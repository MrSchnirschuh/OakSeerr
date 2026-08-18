use crate::db::Database;
use crate::urlencoding;
use crate::models::{Integration, Media};
use crate::services::settings::SettingsService;
use serde_json::Value;
use std::collections::HashMap;

pub struct MediaService;

impl MediaService {
    /// Search across all enabled integrations
    pub async fn search(
        db: &Database,
        query: &str,
        media_type: Option<&str>,
    ) -> anyhow::Result<Vec<Media>> {
        let integrations = db.list_integrations().await?;
        let mut results = Vec::new();

        for integration in &integrations {
            if !integration.enabled {
                continue;
            }

            // Filter by media type if specified
            if let Some(mt) = media_type {
                let expected_type = match mt {
                    "movie" => "radarr",
                    "tv" => "sonarr",
                    "music" => "lidarr",
                    "book" => "readarr",
                    "comic" => "mylar3",
                    _ => continue,
                };
                if integration.integration_type != expected_type {
                    continue;
                }
            }

            if let Ok(items) = Self::search_integration(integration, query).await {
                results.extend(items);
            }
        }

        Ok(results)
    }

    /// Get trending from external APIs (TMDB, LastFM, OpenLibrary, ComicVine)
    pub async fn trending(db: &Database, media_type: Option<&str>) -> anyhow::Result<Vec<Media>> {
        let mut results = Vec::new();

        match media_type {
            Some("movie") | None => {
                if let Ok(movies) = Self::tmdb_trending(db, "movie").await {
                    results.extend(movies);
                }
            }
            _ => {}
        }

        match media_type {
            Some("tv") | None => {
                if let Ok(tv) = Self::tmdb_trending(db, "tv").await {
                    results.extend(tv);
                }
            }
            _ => {}
        }

        match media_type {
            Some("music") | None => {
                if let Ok(music) = Self::lastfm_trending(db).await {
                    results.extend(music);
                }
            }
            _ => {}
        }

        match media_type {
            Some("book") | None => {
                if let Ok(books) = Self::openlibrary_trending().await {
                    results.extend(books);
                }
            }
            _ => {}
        }

        match media_type {
            Some("comic") | None => {
                if let Ok(comics) = Self::comicvine_trending(db).await {
                    results.extend(comics);
                }
            }
            _ => {}
        }

        // Check which items are already in the user's *arr libraries
        let integrations = db.list_integrations().await?;
        for item in &mut results {
            if let Ok(Some(status)) = Self::check_availability(db, &integrations, item).await {
                item.status = status;
            }
        }

        Ok(results)
    }

    /// Get library contents from a specific *arr
    pub async fn library(db: &Database, media_type: &str) -> anyhow::Result<Vec<Media>> {
        let integration_type = match media_type {
            "movie" => "radarr",
            "tv" => "sonarr",
            "music" => "lidarr",
            "book" => "readarr",
            "comic" => "mylar3",
            _ => return Ok(vec![]),
        };

        let integrations = db.list_integrations().await?;
        let mut results = Vec::new();

        for integration in &integrations {
            if !integration.enabled || integration.integration_type != integration_type {
                continue;
            }

            if let Ok(items) = Self::get_library(integration).await {
                results.extend(items);
            }
        }

        Ok(results)
    }

    /// Get detail for a single media item (includes cast, similar items, request status)
    pub async fn detail(db: &Database, media_id: &str) -> anyhow::Result<Option<MediaDetail>> {
        let media = db.get_media(media_id).await?;
        let media = match media {
            Some(m) => m,
            None => return Ok(None),
        };

        // Get request status for this media
        let requests = db.list_requests().await?;
        let request_status =
            requests
                .iter()
                .find(|r| r.media_id == media.id)
                .map(|r| RequestStatusInfo {
                    id: r.id.clone(),
                    status: r.status.clone(),
                    download_status: r.download_status.clone(),
                    created_at: r.created_at.clone(),
                });

        // Try to get cast/similar from TMDB if it's a movie or TV show
        let cast = if let Some(tmdb_id) = media.tmdb_id {
            match media.media_type.as_str() {
                "movie" | "tv" => Self::tmdb_cast(db, media.media_type.as_str(), tmdb_id)
                    .await
                    .ok(),
                _ => None,
            }
        } else {
            None
        };

        let similar = if let Some(tmdb_id) = media.tmdb_id {
            match media.media_type.as_str() {
                "movie" | "tv" => Self::tmdb_similar(db, media.media_type.as_str(), tmdb_id)
                    .await
                    .ok(),
                _ => None,
            }
        } else {
            None
        };

        Ok(Some(MediaDetail {
            media,
            cast: cast.unwrap_or_default(),
            similar: similar.unwrap_or_default(),
            request: request_status,
        }))
    }

    // === Private helpers ===

    async fn search_integration(
        integration: &Integration,
        query: &str,
    ) -> anyhow::Result<Vec<Media>> {
        let client = reqwest::Client::new();
        let encoded = urlencoding(query);

        let (endpoint, param_name) = match integration.integration_type.as_str() {
            "radarr" => ("movie", "term"),
            "sonarr" => ("series", "term"),
            "lidarr" => ("artist", "term"),
            "readarr" => ("book", "term"),
            "mylar3" => ("search", "query"), // Mylar3 uses /api/v3/search?query=
            _ => return Ok(vec![]),
        };

        let url = format!(
            "{}/api/v3/{}?apikey={}&{}={}",
            integration.base_url.trim_end_matches('/'),
            endpoint,
            integration.api_key,
            param_name,
            encoded,
        );

        let resp = client.get(&url).send().await?;
        let data: Vec<Value> = resp.json().await?;

        let items = data
            .into_iter()
            .map(|item| Self::parse_integration_item(&integration.integration_type, &item))
            .filter_map(|r| r.ok())
            .collect();

        Ok(items)
    }

    async fn get_library(integration: &Integration) -> anyhow::Result<Vec<Media>> {
        let client = reqwest::Client::new();
        let endpoint = match integration.integration_type.as_str() {
            "radarr" => "movie",
            "sonarr" => "series",
            "lidarr" => "artist",
            "readarr" => "book",
            "mylar3" => "comics", // Mylar3 uses /api/v3/comics for library
            _ => return Ok(vec![]),
        };

        let url = format!(
            "{}/api/v3/{}?apikey={}",
            integration.base_url.trim_end_matches('/'),
            endpoint,
            integration.api_key,
        );

        let resp = client.get(&url).send().await?;
        let data: Vec<Value> = resp.json().await?;

        let items = data
            .into_iter()
            .map(|item| Self::parse_integration_item(&integration.integration_type, &item))
            .filter_map(|r| r.ok())
            .collect();

        Ok(items)
    }

    fn parse_integration_item(integration_type: &str, item: &Value) -> anyhow::Result<Media> {
        let media_type = match integration_type {
            "radarr" => "movie",
            "sonarr" => "tv",
            "lidarr" => "music",
            "readarr" => "book",
            "mylar3" => "comic",
            _ => return Err(anyhow::anyhow!("Unknown integration type")),
        };

        let title = item
            .get("title")
            .or_else(|| item.get("name"))
            .or_else(|| item.get("artistName"))
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown")
            .to_string();

        let id = item.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
        let _year = item.get("year").and_then(|v| v.as_i64());
        let overview = item
            .get("overview")
            .or_else(|| item.get("description"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        // Build poster URL from integration
        // *arr APIs return 'posterUrl' or 'remotePoster' directly (not in 'images' array)
        // Fall back to 'images' array if direct fields are not present
        let poster_url = if let Some(url) = item.get("posterUrl").and_then(|v| v.as_str()) {
            Some(if url.starts_with("http") {
                url.to_string()
            } else {
                format!("{}{}", integration_type, url)
            })
        } else if let Some(url) = item.get("remotePoster").and_then(|v| v.as_str()) {
            Some(if url.starts_with("http") {
                url.to_string()
            } else {
                format!("{}{}", integration_type, url)
            })
        } else if let Some(url) = item.get("poster").and_then(|v| v.as_str()) {
            Some(if url.starts_with("http") {
                url.to_string()
            } else {
                format!("{}{}", integration_type, url)
            })
        } else if let Some(images) = item.get("images").and_then(|v| v.as_array()) {
            images
                .iter()
                .find(|img| {
                    img.get("coverType")
                        .or_else(|| img.get("type"))
                        .and_then(|v| v.as_str())
                        == Some("poster")
                })
                .or_else(|| images.first())
                .and_then(|img| img.get("url").and_then(|v| v.as_str()))
                .map(|u| {
                    if u.starts_with("http") {
                        u.to_string()
                    } else {
                        format!("{}{}", integration_type, u)
                    }
                })
        } else {
            None
        };

        let backdrop_url = if let Some(images) = item.get("images").and_then(|v| v.as_array()) {
            images
                .iter()
                .find(|img| {
                    img.get("coverType")
                        .or_else(|| img.get("type"))
                        .and_then(|v| v.as_str())
                        == Some("fanart")
                })
                .and_then(|img| img.get("url").and_then(|v| v.as_str()))
                .map(|u| {
                    if u.starts_with("http") {
                        u.to_string()
                    } else {
                        format!("{}{}", integration_type, u)
                    }
                })
        } else {
            None
        };

        let genres: Option<Vec<String>> =
            item.get("genres").and_then(|v| v.as_array()).map(|arr| {
                arr.iter()
                    .filter_map(|g| {
                        if let Some(s) = g.as_str() {
                            Some(s.to_string())
                        } else if let Some(obj) = g.as_object() {
                            obj.get("name")
                                .and_then(|n| n.as_str())
                                .map(|s| s.to_string())
                        } else {
                            None
                        }
                    })
                    .collect()
            });

        let season_count = item
            .get("seasons")
            .and_then(|v| v.as_array())
            .map(|a| a.len() as i32);
        let episode_count = item
            .get("episodeCount")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32);
        let artist_name = item
            .get("artistName")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let author_name = item
            .get("authorName")
            .or_else(|| item.get("author"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let rating = item
            .get("ratings")
            .and_then(|r| r.get("value"))
            .and_then(|v| v.as_f64())
            .or_else(|| item.get("rating").and_then(|v| v.as_f64()));

        let release_date = item
            .get("releaseDate")
            .or_else(|| item.get("release"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        // Serialize genres to JSON string for DB storage
        let genres_json = genres
            .as_ref()
            .map(|g| serde_json::to_string(g).unwrap_or_default());

        Ok(Media {
            id: format!("{}-{}", integration_type, id),
            tmdb_id: item.get("tmdbId").and_then(|v| v.as_i64()),
            tvdb_id: item.get("tvdbId").and_then(|v| v.as_i64()),
            musicbrainz_id: item
                .get("foreignArtistId")
                .or_else(|| item.get("musicBrainzId"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            isbn: item
                .get("isbn")
                .or_else(|| item.get("foreignId"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            media_type: media_type.to_string(),
            title,
            overview,
            poster_url,
            backdrop_url,
            release_date,
            status: "available".to_string(),
            rating,
            genres: genres_json,
            season_count,
            episode_count,
            artist_name,
            author_name,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        })
    }

    async fn check_availability(
        db: &Database,
        integrations: &[Integration],
        media: &Media,
    ) -> anyhow::Result<Option<String>> {
        // Check if this media is already in any *arr library
        let integration_type = match media.media_type.as_str() {
            "movie" => "radarr",
            "tv" => "sonarr",
            "music" => "lidarr",
            "book" => "readarr",
            "comic" => "mylar3",
            _ => return Ok(None),
        };

        for integration in integrations {
            if !integration.enabled || integration.integration_type != integration_type {
                continue;
            }

            let client = reqwest::Client::new();
            let endpoint = match integration_type {
                "radarr" => "movie",
                "sonarr" => "series",
                "lidarr" => "artist",
                "readarr" => "book",
                "mylar3" => "comics",
                _ => continue,
            };

            let search_url = format!(
                "{}/api/v3/{}?apikey={}&term={}",
                integration.base_url.trim_end_matches('/'),
                endpoint,
                integration.api_key,
                urlencoding(&media.title),
            );

            if let Ok(resp) = client.get(&search_url).send().await
                && let Ok(data) = resp.json::<Vec<Value>>().await
                && !data.is_empty()
            {
                return Ok(Some("available".to_string()));
            }
        }

        // Check if it's been requested
        let requests = db.list_requests().await?;
        if requests.iter().any(|r| r.media_id == media.id) {
            return Ok(Some("requested".to_string()));
        }

        Ok(Some("unknown".to_string()))
    }

    // === External API trending ===

    async fn get_api_key(db: &Database, key_name: &str, default: &str) -> String {
        SettingsService::get_api_key(db, key_name, default)
            .await
            .unwrap_or_else(|_| default.to_string())
    }

    async fn tmdb_trending(db: &Database, media_type: &str) -> anyhow::Result<Vec<Media>> {
        let api_key = Self::get_api_key(db, "TMDB_API_KEY", "").await;
        let client = reqwest::Client::new();
        let url = format!(
            "https://api.themoviedb.org/3/trending/{}/week?language=en-US&api_key={}",
            media_type, api_key
        );

        let resp = client
            .get(&url)
            .header("Accept", "application/json")
            .send()
            .await?;

        if !resp.status().is_success() {
            tracing::warn!("TMDB API returned {}", resp.status());
            return Ok(vec![]);
        }

        let data: Value = resp.json().await?;
        let results = data
            .get("results")
            .and_then(|r| r.as_array())
            .ok_or_else(|| anyhow::anyhow!("No results from TMDB"))?;

        // Genre mapping for TMDB genre IDs
        let genre_map: HashMap<i64, &str> = [
            (28, "Action"),
            (12, "Adventure"),
            (16, "Animation"),
            (35, "Comedy"),
            (80, "Crime"),
            (99, "Documentary"),
            (18, "Drama"),
            (10751, "Family"),
            (14, "Fantasy"),
            (36, "History"),
            (27, "Horror"),
            (10402, "Music"),
            (9648, "Mystery"),
            (10749, "Romance"),
            (878, "Sci-Fi"),
            (10770, "TV Movie"),
            (53, "Thriller"),
            (10752, "War"),
            (37, "Western"),
            // TV-specific
            (10759, "Action & Adventure"),
            (10762, "Kids"),
            (10763, "News"),
            (10764, "Reality"),
            (10765, "Sci-Fi & Fantasy"),
            (10766, "Soap"),
            (10767, "Talk"),
            (10768, "War & Politics"),
        ]
        .iter()
        .cloned()
        .collect();

        let items = results
            .iter()
            .map(|item| {
                let id = item.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
                let title = item
                    .get("title")
                    .or_else(|| item.get("name"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown")
                    .to_string();
                let overview = item
                    .get("overview")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                let poster_path = item.get("poster_path").and_then(|v| v.as_str());
                let backdrop_path = item.get("backdrop_path").and_then(|v| v.as_str());
                let release_date = item
                    .get("release_date")
                    .or_else(|| item.get("first_air_date"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                let vote_average = item.get("vote_average").and_then(|v| v.as_f64());

                let genres: Option<Vec<String>> =
                    item.get("genre_ids").and_then(|v| v.as_array()).map(|arr| {
                        arr.iter()
                            .filter_map(|g| {
                                g.as_i64()
                                    .and_then(|id| genre_map.get(&id))
                                    .map(|s| s.to_string())
                            })
                            .collect()
                    });

                let genres_json = genres
                    .as_ref()
                    .map(|g| serde_json::to_string(g).unwrap_or_default());

                Media {
                    id: format!("tmdb-{}", id),
                    tmdb_id: Some(id),
                    tvdb_id: None,
                    musicbrainz_id: None,
                    isbn: None,
                    media_type: media_type.to_string(),
                    title,
                    overview,
                    poster_url: poster_path
                        .map(|p| format!("https://image.tmdb.org/t/p/w500{}", p)),
                    backdrop_url: backdrop_path
                        .map(|p| format!("https://image.tmdb.org/t/p/w1280{}", p)),
                    release_date,
                    status: "unknown".to_string(),
                    rating: vote_average,
                    genres: genres_json,
                    season_count: None,
                    episode_count: None,
                    artist_name: None,
                    author_name: None,
                    created_at: chrono::Utc::now().to_rfc3339(),
                    updated_at: chrono::Utc::now().to_rfc3339(),
                }
            })
            .collect();

        Ok(items)
    }

    async fn tmdb_cast(
        db: &Database,
        media_type: &str,
        tmdb_id: i64,
    ) -> anyhow::Result<Vec<CastMember>> {
        let api_key = Self::get_api_key(db, "TMDB_API_KEY", "").await;
        let client = reqwest::Client::new();
        let url = format!(
            "https://api.themoviedb.org/3/{}/{}/credits?api_key={}&language=en-US",
            if media_type == "movie" { "movie" } else { "tv" },
            tmdb_id,
            api_key,
        );

        let resp = client.get(&url).send().await?;
        if !resp.status().is_success() {
            return Ok(vec![]);
        }

        let data: Value = resp.json().await?;
        let cast = data
            .get("cast")
            .and_then(|c| c.as_array())
            .map(|arr| {
                arr.iter()
                    .take(20)
                    .filter_map(|c| {
                        Some(CastMember {
                            name: c.get("name").and_then(|v| v.as_str())?.to_string(),
                            character: c
                                .get("character")
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string()),
                            profile_path: c
                                .get("profile_path")
                                .and_then(|v| v.as_str())
                                .map(|p| format!("https://image.tmdb.org/t/p/w185{}", p)),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        Ok(cast)
    }

    async fn tmdb_similar(
        db: &Database,
        media_type: &str,
        tmdb_id: i64,
    ) -> anyhow::Result<Vec<Media>> {
        let api_key = Self::get_api_key(db, "TMDB_API_KEY", "").await;
        let client = reqwest::Client::new();
        let url = format!(
            "https://api.themoviedb.org/3/{}/{}/similar?api_key={}&language=en-US&page=1",
            if media_type == "movie" { "movie" } else { "tv" },
            tmdb_id,
            api_key,
        );

        let resp = client.get(&url).send().await?;
        if !resp.status().is_success() {
            return Ok(vec![]);
        }

        let data: Value = resp.json().await?;
        let results = data
            .get("results")
            .and_then(|r| r.as_array())
            .map(|arr| {
                arr.iter()
                    .take(10)
                    .filter_map(|item| {
                        let id = item.get("id").and_then(|v| v.as_i64())?;
                        let title = item
                            .get("title")
                            .or_else(|| item.get("name"))
                            .and_then(|v| v.as_str())?
                            .to_string();
                        let poster_path = item.get("poster_path").and_then(|v| v.as_str());
                        Some(Media {
                            id: format!("tmdb-{}", id),
                            tmdb_id: Some(id),
                            tvdb_id: None,
                            musicbrainz_id: None,
                            isbn: None,
                            media_type: media_type.to_string(),
                            title,
                            overview: item
                                .get("overview")
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string()),
                            poster_url: poster_path
                                .map(|p| format!("https://image.tmdb.org/t/p/w500{}", p)),
                            backdrop_url: None,
                            release_date: item
                                .get("release_date")
                                .or_else(|| item.get("first_air_date"))
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string()),
                            status: "unknown".to_string(),
                            rating: item.get("vote_average").and_then(|v| v.as_f64()),
                            genres: None,
                            season_count: None,
                            episode_count: None,
                            artist_name: None,
                            author_name: None,
                            created_at: chrono::Utc::now().to_rfc3339(),
                            updated_at: chrono::Utc::now().to_rfc3339(),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        Ok(results)
    }

    async fn lastfm_trending(db: &Database) -> anyhow::Result<Vec<Media>> {
        let api_key = Self::get_api_key(db, "LASTFM_API_KEY", "").await;
        let client = reqwest::Client::new();
        let url = format!(
            "https://ws.audioscrobbler.com/2.0/?method=chart.gettopartists&api_key={}&format=json&limit=20",
            api_key
        );

        let resp = client.get(&url).send().await?;
        if !resp.status().is_success() {
            tracing::warn!("LastFM API returned {}", resp.status());
            return Ok(vec![]);
        }

        let data: Value = resp.json().await?;
        let artists = data
            .get("artists")
            .and_then(|a| a.get("artist"))
            .and_then(|a| a.as_array())
            .ok_or_else(|| anyhow::anyhow!("No artists from LastFM"))?;

        let items = artists
            .iter()
            .map(|item| {
                let name = item
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown")
                    .to_string();
                let mbid = item
                    .get("mbid")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                let image = item
                    .get("image")
                    .and_then(|v| v.as_array())
                    .and_then(|imgs| {
                        imgs.iter()
                            .find(|img| img.get("size").and_then(|s| s.as_str()) == Some("large"))
                    })
                    .and_then(|img| img.get("#text").and_then(|v| v.as_str()))
                    .map(|s| s.to_string());

                Media {
                    id: format!("lastfm-{}", name.to_lowercase().replace(' ', "-")),
                    tmdb_id: None,
                    tvdb_id: None,
                    musicbrainz_id: mbid,
                    isbn: None,
                    media_type: "music".to_string(),
                    title: name,
                    overview: None,
                    poster_url: image.filter(|u| !u.is_empty()),
                    backdrop_url: None,
                    release_date: None,
                    status: "unknown".to_string(),
                    rating: None,
                    genres: None,
                    season_count: None,
                    episode_count: None,
                    artist_name: None,
                    author_name: None,
                    created_at: chrono::Utc::now().to_rfc3339(),
                    updated_at: chrono::Utc::now().to_rfc3339(),
                }
            })
            .collect();

        Ok(items)
    }

    async fn openlibrary_trending() -> anyhow::Result<Vec<Media>> {
        let client = reqwest::Client::new();
        let url = "https://openlibrary.org/trending/daily.json?limit=20";

        let resp = client.get(url).send().await?;
        if !resp.status().is_success() {
            tracing::warn!("OpenLibrary API returned {}", resp.status());
            return Ok(vec![]);
        }

        let data: Value = resp.json().await?;
        let works = data
            .get("works")
            .and_then(|w| w.as_array())
            .ok_or_else(|| anyhow::anyhow!("No works from OpenLibrary"))?;

        let items = works
            .iter()
            .map(|item| {
                let title = item
                    .get("title")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown")
                    .to_string();
                let key = item
                    .get("key")
                    .and_then(|v| v.as_str())
                    .unwrap_or("/works/OL0W");
                let id = key.trim_start_matches("/works/");
                let cover_id = item.get("cover_id").and_then(|v| v.as_i64());
                let authors = item
                    .get("authors")
                    .and_then(|v| v.as_array())
                    .and_then(|arr| arr.first())
                    .and_then(|a| a.get("name").and_then(|v| v.as_str()))
                    .map(|s| s.to_string());
                let first_publish_year = item.get("first_publish_year").and_then(|v| v.as_i64());

                Media {
                    id: format!("ol-{}", id),
                    tmdb_id: None,
                    tvdb_id: None,
                    musicbrainz_id: None,
                    isbn: item
                        .get("isbn")
                        .and_then(|v| v.as_array())
                        .and_then(|arr| arr.first())
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    media_type: "book".to_string(),
                    title,
                    overview: authors.clone(),
                    poster_url: cover_id
                        .map(|c| format!("https://covers.openlibrary.org/b/id/{}-L.jpg", c)),
                    backdrop_url: None,
                    release_date: first_publish_year.map(|y| y.to_string()),
                    status: "unknown".to_string(),
                    rating: None,
                    genres: None,
                    season_count: None,
                    episode_count: None,
                    artist_name: None,
                    author_name: authors,
                    created_at: chrono::Utc::now().to_rfc3339(),
                    updated_at: chrono::Utc::now().to_rfc3339(),
                }
            })
            .collect();

        Ok(items)
    }

    async fn comicvine_trending(db: &Database) -> anyhow::Result<Vec<Media>> {
        let api_key = Self::get_api_key(db, "COMICVINE_API_KEY", "").await;
        let client = reqwest::Client::new();
        let url = format!(
            "https://comicvine.gamespot.com/api/issues/?api_key={}&format=json&sort=date_added:desc&limit=20&field_list=id,name,image,volume,cover_date,description",
            api_key
        );

        let resp = client.get(&url).send().await?;
        if !resp.status().is_success() {
            tracing::warn!("ComicVine API returned {}", resp.status());
            return Ok(vec![]);
        }

        let data: Value = resp.json().await?;
        let results = data
            .get("results")
            .and_then(|r| r.as_array())
            .ok_or_else(|| anyhow::anyhow!("No results from ComicVine"))?;

        let items = results
            .iter()
            .map(|item| {
                let name = item
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown")
                    .to_string();
                let id = item.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
                let image = item
                    .get("image")
                    .and_then(|v| v.get("super_url"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                let volume = item
                    .get("volume")
                    .and_then(|v| v.get("name"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                let cover_date = item
                    .get("cover_date")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                let description = item
                    .get("description")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());

                let title = if let Some(v) = volume {
                    format!("{} #{}", v, name)
                } else {
                    name
                };

                Media {
                    id: format!("cv-{}", id),
                    tmdb_id: None,
                    tvdb_id: None,
                    musicbrainz_id: None,
                    isbn: None,
                    media_type: "comic".to_string(),
                    title,
                    overview: description,
                    poster_url: image,
                    backdrop_url: None,
                    release_date: cover_date,
                    status: "unknown".to_string(),
                    rating: None,
                    genres: None,
                    season_count: None,
                    episode_count: None,
                    artist_name: None,
                    author_name: None,
                    created_at: chrono::Utc::now().to_rfc3339(),
                    updated_at: chrono::Utc::now().to_rfc3339(),
                }
            })
            .collect();

        Ok(items)
    }
}

// === Data structures for detail endpoint ===

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MediaDetail {
    pub media: Media,
    pub cast: Vec<CastMember>,
    pub similar: Vec<Media>,
    pub request: Option<RequestStatusInfo>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CastMember {
    pub name: String,
    pub character: Option<String>,
    pub profile_path: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RequestStatusInfo {
    pub id: String,
    pub status: String,
    pub download_status: String,
    pub created_at: String,
}
