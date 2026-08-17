"use client";

import Link from "next/link";
import Image from "next/image";
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

const typeIcons: Record<string, React.ComponentType<{ size?: number; strokeWidth?: number; style?: React.CSSProperties }>> = {
  movie: Film,
  tv: Tv,
  music: Music,
  book: BookOpen,
  comic: BookMarked,
};

const statusConfig: Record<string, { icon: React.ComponentType<{ size?: number; style?: React.CSSProperties }>; label: string; className: string }> = {
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
    <Link
      href={`/detail?id=${encodeURIComponent(item.id)}`}
      className="media-card"
      style={{ textDecoration: "none", color: "inherit", display: "block" }}
    >
      <div className="media-card-poster">
        {item.poster_url ? (
          <Image
            src={item.poster_url}
            alt={item.title}
            width={300}
            height={450}
            style={{ width: "100%", height: "auto", display: "block" }}
            unoptimized
            onError={(e) => {
              const target = e.currentTarget as HTMLImageElement;
              target.style.display = "none";
              target.parentElement!.querySelector(".media-card-poster-placeholder")!.classList.remove("hidden");
            }}
          />
        ) : null}
        <div className={`media-card-poster-placeholder ${item.poster_url ? "hidden" : ""}`}>
          <TypeIcon size={48} strokeWidth={1} />
        </div>

        {/* Status badge with backdrop-filter + text-shadow for readability */}
        <div className="media-card-status">
          <span className={`badge ${status.className}`} style={{ backdropFilter: "blur(8px)", WebkitBackdropFilter: "blur(8px)", textShadow: "0 1px 4px rgba(0,0,0,0.6)" }}>
            <StatusIcon size={12} />
            {status.label}
          </span>
        </div>

        {/* Genre badges on poster */}
        {item.genres && item.genres.length > 0 && (
          <div className="media-card-genres">
            {item.genres.slice(0, 3).map((genre) => (
              <span key={genre} className="badge genre-badge">{genre}</span>
            ))}
          </div>
        )}

        {/* Hover overlay */}
        <div className="media-card-overlay">
          {item.overview && (
            <p className="media-card-overview" style={{ fontSize: "0.75rem", lineHeight: 1.4, color: "rgba(255,255,255,0.85)", overflow: "hidden", textOverflow: "ellipsis", display: "-webkit-box", WebkitLineClamp: 4, WebkitBoxOrient: "vertical" }}>
              {item.overview}
            </p>
          )}
          {item.rating && (
            <div className="media-card-rating" style={{ display: "flex", alignItems: "center", gap: "4px", marginTop: "8px", fontSize: "0.8rem", color: "var(--jf-star)" }}>
              <span>★</span> {item.rating.toFixed(1)}
            </div>
          )}
        </div>
      </div>

      <div className="media-card-info">
        <div className="media-card-title">{item.title}</div>
        {item.year && <div className="media-card-year">{item.year}</div>}
        {item.artistName && <div className="media-card-subtitle" style={{ fontSize: "0.7rem", color: "var(--jf-text-secondary)" }}>{item.artistName}</div>}
        {item.authorName && <div className="media-card-subtitle" style={{ fontSize: "0.7rem", color: "var(--jf-text-secondary)" }}>{item.authorName}</div>}
      </div>
    </Link>
  );
}
