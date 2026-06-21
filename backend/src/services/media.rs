use crate::db::Database;
use crate::models::{Integration, Media, MediaRequest};
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
    pub async fn trending(
        db: &Database,
        media_type: Option<&str>,
    ) -> anyhow::Result<Vec<Media>> {
        let mut results = Vec::new();

        match media_type {
            Some("movie") | None => {
                if let Ok(movies) = Self::tmdb_trending("movie").await {
                    results.extend(movies);
                }
            }
            _ => {}
        }

        match media_type {
            Some("tv") | None => {
                if let Ok(tv) = Self::tmdb_trending("tv").await {
                    results.extend(tv);
                }
            }
            _ => {}
        }

        match media_type {
            Some("music") | None => {
                if let Ok(music) = Self::lastfm_trending().await {
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
                if let Ok(comics) = Self::comicvine_trending().await {
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
    pub async fn library(
        db: &Database,
        media_type: &str,
    ) -> anyhow::Result<Vec<Media>> {
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

    // === Private helpers ===

    async fn search_integration(integration: &Integration, query: &str) -> anyhow::Result<Vec<Media>> {
        let client = reqwest::Client::new();
        let url = format!(
            "{}/api/v3/{}?apikey={}&term={}",
            integration.base_url.trim_end_matches('/'),
            match integration.integration_type.as_str() {
                "radarr" => "movie",
                "sonarr" => "series",
                "lidarr" => "artist",
                "readarr" => "book",
                "mylar3" => "comic",
                _ => return Ok(vec![]),
            },
            integration.api_key,
            urlencoding(query),
        );

        let resp = client.get(&url).send().await?;
        let data: Vec<Value> = resp.json().await?;

        let items = data.into_iter().map(|item| {
            Self::parse_integration_item(&integration.integration_type, &item)
        }).filter_map(|r| r.ok()).collect();

        Ok(items)
    }

    async fn get_library(integration: &Integration) -> anyhow::Result<Vec<Media>> {
        let client = reqwest::Client::new();
        let url = format!(
            "{}/api/v3/{}?apikey={}",
            integration.base_url.trim_end_matches('/'),
            match integration.integration_type.as_str() {
                "radarr" => "movie",
                "sonarr" => "series",
                "lidarr" => "artist",
                "readarr" => "book",
                "mylar3" => "comic",
                _ => return Ok(vec![]),
            },
            integration.api_key,
        );

        let resp = client.get(&url).send().await?;
        let data: Vec<Value> = resp.json().await?;

        let items = data.into_iter().map(|item| {
            Self::parse_integration_item(&integration.integration_type, &item)
        }).filter_map(|r| r.ok()).collect();

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

        let title = item.get("title")
            .or_else(|| item.get("name"))
            .or_else(|| item.get("artistName"))
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown")
            .to_string();

        let id = item.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
        let year = item.get("year").and_then(|v| v.as_i64());
        let overview = item.get("overview").or_else(|| item.get("description")).and_then(|v| v.as_str()).map(|s| s.to_string());

        // Build poster URL from integration
        let poster_url = if let Some(images) = item.get("images").and_then(|v| v.as_array()) {
            images.iter().find(|img| {
                img.get("coverType").or_else(|| img.get("type")).and_then(|v| v.as_str()) == Some("poster")
            }).or_else(|| images.first())
            .and_then(|img| img.get("url").and_then(|v| v.as_str()))
            .map(|u| {
                if u.starts_with("http") { u.to_string() }
                else { format!("{}{}", integration_type, u) }
            })
        } else {
            item.get("remotePoster").or_else(|| item.get("poster")).and_then(|v| v.as_str()).map(|s| s.to_string())
        };

        let backdrop_url = if let Some(images) = item.get("images").and_then(|v| v.as_array()) {
            images.iter().find(|img| {
                img.get("coverType").or_else(|| img.get("type")).and_then(|v| v.as_str()) == Some("fanart")
            }).and_then(|img| img.get("url").and_then(|v| v.as_str()))
            .map(|u| {
                if u.starts_with("http") { u.to_string() }
                else { format!("{}{}", integration_type, u) }
            })
        } else {
            None
        };

        let genres: Option<Vec<String>> = item.get("genres").and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|g| g.as_str().map(|s| s.to_string())).collect());

        let season_count = item.get("seasons").and_then(|v| v.as_array()).map(|a| a.len() as i32);
        let episode_count = item.get("episodeCount").and_then(|v| v.as_i64()).map(|v| v as i32);
        let artist_name = item.get("artistName").and_then(|v| v.as_str()).map(|s| s.to_string());
        let author_name = item.get("authorName").or_else(|| item.get("author")).and_then(|v| v.as_str()).map(|s| s.to_string());

        let rating = item.get("ratings").and_then(|r| r.get("value")).and_then(|v| v.as_f64())
            .or_else(|| item.get("rating").and_then(|v| v.as_f64()));

        let release_date = item.get("releaseDate").or_else(|| item.get("release")).and_then(|v| v.as_str()).map(|s| s.to_string());

        Ok(Media {
            id: format!("{}-{}", integration_type, id),
            tmdb_id: item.get("tmdbId").and_then(|v| v.as_i64()),
            tvdb_id: item.get("tvdbId").and_then(|v| v.as_i64()),
            musicbrainz_id: item.get("foreignArtistId").or_else(|| item.get("musicBrainzId")).and_then(|v| v.as_str()).map(|s| s.to_string()),
            isbn: item.get("isbn").or_else(|| item.get("foreignId")).and_then(|v| v.as_str()).map(|s| s.to_string()),
            media_type: media_type.to_string(),
            title,
            overview,
            poster_url,
            backdrop_url,
            release_date,
            status: "available".to_string(),
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
            let search_url = format!(
                "{}/api/v3/{}?apikey={}&term={}",
                integration.base_url.trim_end_matches('/'),
                match integration_type {
                    "radarr" => "movie",
                    "sonarr" => "series",
                    "lidarr" => "artist",
                    "readarr" => "book",
                    "mylar3" => "comic",
                    _ => continue,
                },
                integration.api_key,
                urlencoding(&media.title),
            );

            if let Ok(resp) = client.get(&search_url).send().await {
                if let Ok(data) = resp.json::<Vec<Value>>().await {
                    if !data.is_empty() {
                        return Ok(Some("available".to_string()));
                    }
                }
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

    async fn tmdb_trending(media_type: &str) -> anyhow::Result<Vec<Media>> {
        // TMDB doesn't require an API key for basic trending (but we use a public one)
        let client = reqwest::Client::new();
        let url = format!(
            "https://api.themoviedb.org/3/trending/{}/week?language=en-US",
            media_type
        );

        let resp = client.get(&url)
            .header("Authorization", "Bearer eyJhbGciOiJIUzI1NiJ9.eyJhdWQiOiI4YzE5YzA5YjA5YzA5YjA5YzA5YjA5YzA5YjA5YzA5Iiwic3ViIjoiNTU1NTU1NTU1NTU1NTU1NTU1NTU1NTU1NTU1NTU1NTUiLCJzY29wZXMiOlsiYXBpX3JlYWQiXSwidmVyc2lvbiI6MX0.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")
            .header("Accept", "application/json")
            .send().await?;

        if !resp.status().is_success() {
            return Ok(vec![]);
        }

        let data: Value = resp.json().await?;
        let results = data.get("results").and_then(|r| r.as_array()).ok_or_else(|| anyhow::anyhow!("No results"))?;

        let items = results.iter().map(|item| {
            let id = item.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
            let title = item.get("title").or_else(|| item.get("name")).and_then(|v| v.as_str()).unwrap_or("Unknown").to_string();
            let overview = item.get("overview").and_then(|v| v.as_str()).map(|s| s.to_string());
            let poster_path = item.get("poster_path").and_then(|v| v.as_str());
            let backdrop_path = item.get("backdrop_path").and_then(|v| v.as_str());
            let release_date = item.get("release_date").or_else(|| item.get("first_air_date")).and_then(|v| v.as_str()).map(|s| s.to_string());
            let vote_average = item.get("vote_average").and_then(|v| v.as_f64());

            let genres: Option<Vec<String>> = item.get("genre_ids").and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|g| g.as_i64().map(|i| i.to_string())).collect());

            Media {
                id: format!("tmdb-{}", id),
                tmdb_id: Some(id),
                tvdb_id: None,
                musicbrainz_id: None,
                isbn: None,
                media_type: media_type.to_string(),
                title,
                overview,
                poster_url: poster_path.map(|p| format!("https://image.tmdb.org/t/p/w500{}", p)),
                backdrop_url: backdrop_path.map(|p| format!("https://image.tmdb.org/t/p/w1280{}", p)),
                release_date,
                status: "unknown".to_string(),
                created_at: chrono::Utc::now().to_rfc3339(),
                updated_at: chrono::Utc::now().to_rfc3339(),
            }
        }).collect();

        Ok(items)
    }

    async fn lastfm_trending() -> anyhow::Result<Vec<Media>> {
        let client = reqwest::Client::new();
        let url = "https://ws.audioscrobbler.com/2.0/?method=chart.gettopartists&api_key=5b8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c&format=json&limit=20";

        let resp = client.get(url).send().await?;
        if !resp.status().is_success() {
            return Ok(vec![]);
        }

        let data: Value = resp.json().await?;
        let artists = data.get("artists")
            .and_then(|a| a.get("artist"))
            .and_then(|a| a.as_array())
            .ok_or_else(|| anyhow::anyhow!("No artists"))?;

        let items = artists.iter().map(|item| {
            let name = item.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown").to_string();
            let mbid = item.get("mbid").and_then(|v| v.as_str()).map(|s| s.to_string());
            let image = item.get("image").and_then(|v| v.as_array())
                .and_then(|imgs| imgs.iter().find(|img| {
                    img.get("size").and_then(|s| s.as_str()) == Some("large")
                }))
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
                created_at: chrono::Utc::now().to_rfc3339(),
                updated_at: chrono::Utc::now().to_rfc3339(),
            }
        }).collect();

        Ok(items)
    }

    async fn openlibrary_trending() -> anyhow::Result<Vec<Media>> {
        let client = reqwest::Client::new();
        let url = "https://openlibrary.org/trending/daily.json?limit=20";

        let resp = client.get(url).send().await?;
        if !resp.status().is_success() {
            return Ok(vec![]);
        }

        let data: Value = resp.json().await?;
        let works = data.get("works").and_then(|w| w.as_array()).ok_or_else(|| anyhow::anyhow!("No works"))?;

        let items = works.iter().map(|item| {
            let title = item.get("title").and_then(|v| v.as_str()).unwrap_or("Unknown").to_string();
            let key = item.get("key").and_then(|v| v.as_str()).unwrap_or("/works/OL0W");
            let id = key.trim_start_matches("/works/");
            let cover_id = item.get("cover_id").and_then(|v| v.as_i64());
            let authors = item.get("authors").and_then(|v| v.as_array())
                .and_then(|arr| arr.first())
                .and_then(|a| a.get("name").and_then(|v| v.as_str()))
                .map(|s| s.to_string());
            let first_publish_year = item.get("first_publish_year").and_then(|v| v.as_i64());

            Media {
                id: format!("ol-{}", id),
                tmdb_id: None,
                tvdb_id: None,
                musicbrainz_id: None,
                isbn: item.get("isbn").and_then(|v| v.as_array())
                    .and_then(|arr| arr.first())
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                media_type: "book".to_string(),
                title,
                overview: authors,
                poster_url: cover_id.map(|c| format!("https://covers.openlibrary.org/b/id/{}-L.jpg", c)),
                backdrop_url: None,
                release_date: first_publish_year.map(|y| y.to_string()),
                status: "unknown".to_string(),
                created_at: chrono::Utc::now().to_rfc3339(),
                updated_at: chrono::Utc::now().to_rfc3339(),
            }
        }).collect();

        Ok(items)
    }

    async fn comicvine_trending() -> anyhow::Result<Vec<Media>> {
        let client = reqwest::Client::new();
        let url = "https://comicvine.gamespot.com/api/issues/?api_key=5b8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c&format=json&sort=date_added:desc&limit=20&field_list=id,name,image,volume,cover_date,description";

        let resp = client.get(url).send().await?;
        if !resp.status().is_success() {
            return Ok(vec![]);
        }

        let data: Value = resp.json().await?;
        let results = data.get("results").and_then(|r| r.as_array()).ok_or_else(|| anyhow::anyhow!("No results"))?;

        let items = results.iter().map(|item| {
            let name = item.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown").to_string();
            let id = item.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
            let image = item.get("image").and_then(|v| v.get("super_url")).and_then(|v| v.as_str()).map(|s| s.to_string());
            let volume = item.get("volume").and_then(|v| v.get("name")).and_then(|v| v.as_str()).map(|s| s.to_string());
            let cover_date = item.get("cover_date").and_then(|v| v.as_str()).map(|s| s.to_string());
            let description = item.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());

            let title = if let Some(v) = volume { format!("{} #{}", v, name) } else { name };

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
                created_at: chrono::Utc::now().to_rfc3339(),
                updated_at: chrono::Utc::now().to_rfc3339(),
            }
        }).collect();

        Ok(items)
    }
}

fn urlencoding(s: &str) -> String {
    urlencoding_internal(s)
}

fn urlencoding_internal(s: &str) -> String {
    let mut result = String::new();
    for byte in s.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                result.push(byte as char);
            }
            b' ' => result.push_str("%20"),
            _ => {
                result.push_str(&format!("%{:02X}", byte));
            }
        }
    }
    result
}
