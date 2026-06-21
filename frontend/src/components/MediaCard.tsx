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
}

const typeIcons: Record<string, any> = {
  movie: Film,
  tv: Tv,
  music: Music,
  book: BookOpen,
  comic: BookMarked,
};

const statusConfig: Record<string, { icon: any; label: string; className: string }> = {
  available: { icon: CheckCircle2, label: "Available", className: "badge-success" },
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
      {/* Poster */}
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

        {/* Status badge */}
        <div className="media-card-status">
          <span className={`badge ${status.className}`}>
            <StatusIcon size={12} />
            {status.label}
          </span>
        </div>

        {/* Hover overlay */}
        <div className="media-card-overlay">
          {item.overview && (
            <p style={{ fontSize: "0.75rem", lineHeight: 1.4, color: "rgba(255,255,255,0.8)", display: "-webkit-box", WebkitLineClamp: 4, WebkitBoxOrient: "vertical", overflow: "hidden" }}>
              {item.overview}
            </p>
          )}
        </div>
      </div>

      {/* Info */}
      <div className="media-card-info">
        <div className="media-card-title">{item.title}</div>
        {item.year && <div className="media-card-year">{item.year}</div>}
      </div>
    </div>
  );
}
