"use client";

import { Film, Tv, Music, BookOpen, BookMarked, CheckCircle2, Clock, AlertCircle, XCircle } from "lucide-react";

interface MediaItem {
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
}

const typeIcons: Record<string, any> = {
  movie: Film,
  tv: Tv,
  music: Music,
  book: BookOpen,
  comic: BookMarked,
};

const statusConfig: Record<string, { icon: any; label: string; className: string }> = {
  available: { icon: CheckCircle2, label: "Available", className: "badge-available" },
  requested: { icon: Clock, label: "Requested", className: "badge-primary" },
  processing: { icon: AlertCircle, label: "Processing", className: "badge-warning" },
  error: { icon: XCircle, label: "Error", className: "badge-error" },
};

export default function MediaCard({ item }: { item: MediaItem }) {
  const TypeIcon = typeIcons[item.media_type] || Film;
  const status = statusConfig[item.status] || { icon: Clock, label: item.status, className: "badge" };
  const StatusIcon = status.icon;

  return (
    <div
      className="media-card"
      onClick={() => window.location.href = `/${item.media_type}/${item.id}`}
    >
      <div className="media-card-poster">
        {item.poster_url ? (
          <img
            src={item.poster_url}
            alt={item.title}
            loading="lazy"
            onError={(e) => {
              (e.target as HTMLImageElement).style.display = "none";
              (e.target as HTMLImageElement).parentElement!.querySelector(".placeholder")!.classList.remove("hidden");
            }}
          />
        ) : null}
        <div className={`media-card-poster-placeholder ${item.poster_url ? "hidden" : ""}`}>
          <TypeIcon size={48} strokeWidth={1} />
        </div>

        {/* Status badge — improved readability with backdrop-blur + text-shadow */}
        <div className="media-card-status">
          <span className={`badge ${status.className}`}>
            <StatusIcon size={12} />
            {status.label}
          </span>
        </div>

        {/* Hover overlay with gradient */}
        <div className="media-card-overlay">
          {item.overview && (
            <p className="media-card-overview">
              {item.overview}
            </p>
          )}
          {item.rating && (
            <div className="media-card-rating">
              <span>★</span> {item.rating.toFixed(1)}
            </div>
          )}
        </div>
      </div>

      <div className="media-card-info">
        <div className="media-card-title">{item.title}</div>
        {item.year && <div className="media-card-year">{item.year}</div>}
        {item.artistName && <div className="media-card-subtitle">{item.artistName}</div>}
        {item.authorName && <div className="media-card-subtitle">{item.authorName}</div>}
      </div>
    </div>
  );
}
