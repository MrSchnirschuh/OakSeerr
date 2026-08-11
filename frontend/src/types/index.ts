export interface MediaItem {
  id: string;
  title: string;
  year?: number;
  media_type: string;
  poster_url: string | null;
  backdrop_url?: string | null;
  overview?: string | null;
  status: string;
  rating?: number;
  genres?: string[];
  seasonCount?: number;
  episodeCount?: number;
  artistName?: string;
  authorName?: string;
  cast?: { name: string; role: string; image: string | null }[];
  requester?: string;
  created_at?: string;
}
