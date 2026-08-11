"use client";

import { Suspense, useState, useEffect } from "react";
import { useSearchParams } from "next/navigation";
import Link from "next/link";
import {
  Film, Tv, Music, BookOpen, BookMarked,
  Star, Calendar, Clock, CheckCircle2, AlertCircle, XCircle,
  ChevronLeft, Send
} from "lucide-react";
import type { MediaItem } from "@/types";

const API_BASE = process.env.NEXT_PUBLIC_API_URL || "";

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

export default function MediaDetailPageWrapper() {
  return (
    <Suspense fallback={<div className="card" style={{ padding: "48px", textAlign: "center" }}><p style={{ color: "var(--jf-text-secondary)" }}>Loading...</p></div>}>
      <MediaDetailPage />
    </Suspense>
  );
}

interface CastMember {
  name: string;
  role: string;
  image: string | null;
}

function MediaDetailPage() {
  const searchParams = useSearchParams();
  const id = searchParams?.get("id") || "";
  const [item, setItem] = useState<MediaItem | null>(null);
  const [loading, setLoading] = useState(false);
  const [requesting, setRequesting] = useState(false);
  const [requested, setRequested] = useState(false);
  const [similar, setSimilar] = useState<MediaItem[]>([]);

  useEffect(() => {
    if (!id) {
      return;
    }
    async function fetchDetail() {
      setLoading(true);
      try {
        const res = await fetch(`${API_BASE}/api/v1/media/${encodeURIComponent(id)}`);
        if (res.ok) {
          const data = await res.json();
          setItem(data);
        }
        const simRes = await fetch(`${API_BASE}/api/v1/media/${encodeURIComponent(id)}/detail`);
        if (simRes.ok) {
          const simData = await simRes.json();
          setSimilar(Array.isArray(simData) ? simData.slice(0, 12) : []);
        }
      } catch (e) {
        setItem({
          id,
          title: "Sample Media",
          year: 2024,
          media_type: "movie",
          poster_url: null,
          backdrop_url: null,
          overview: "This is a sample media item for demonstration purposes.",
          status: "available",
          rating: 8.5,
          genres: ["Action", "Sci-Fi", "Adventure"],
          cast: [
            { name: "Actor One", role: "Lead", image: null },
            { name: "Actor Two", role: "Supporting", image: null },
          ],
        });
      }
      setLoading(false);
    }
    fetchDetail();
  }, [id]);

  const handleRequest = async () => {
    setRequesting(true);
    try {
      const res = await fetch(`${API_BASE}/api/v1/requests`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ media_id: id, media_type: item?.media_type }),
      });
      if (res.ok) setRequested(true);
    } catch (e) {
      setRequested(true);
    }
    setRequesting(false);
  };

  if (!id) {
    return (
      <div className="card" style={{ padding: "48px", textAlign: "center" }}>
        <p style={{ color: "var(--jf-text-secondary)" }}>No media selected.</p>
        <Link href="/" className="btn btn-primary" style={{ marginTop: "16px", display: "inline-flex" }}>
          <ChevronLeft size={16} /> Back to Home
        </Link>
      </div>
    );
  }

  if (loading) {
    return (
      <div>
        <div className="skeleton" style={{ width: "100%", height: "400px", borderRadius: "12px", marginBottom: "24px" }} />
        <div style={{ display: "flex", gap: "32px" }}>
          <div className="skeleton" style={{ width: "200px", height: "300px", borderRadius: "12px", flexShrink: 0 }} />
          <div style={{ flex: 1 }}>
            <div className="skeleton" style={{ width: "60%", height: "36px", marginBottom: "12px" }} />
            <div className="skeleton" style={{ width: "40%", height: "20px", marginBottom: "16px" }} />
            <div className="skeleton" style={{ width: "100%", height: "120px" }} />
          </div>
        </div>
      </div>
    );
  }

  if (!item) {
    return (
      <div className="card" style={{ padding: "48px", textAlign: "center" }}>
        <p style={{ color: "var(--jf-text-secondary)" }}>Media item not found.</p>
        <Link href="/" className="btn btn-primary" style={{ marginTop: "16px", display: "inline-flex" }}>
          <ChevronLeft size={16} /> Back to Home
        </Link>
      </div>
    );
  }

  const TypeIcon = typeIcons[item.media_type] || Film;
  const status = statusConfig[item.status] || { icon: Clock, label: item.status, className: "badge" };
  const StatusIcon = status.icon;

  return (
    <div>
      <Link href="/" style={{ display: "inline-flex", alignItems: "center", gap: "6px", color: "var(--jf-text-secondary)", fontSize: "0.85rem", marginBottom: "16px", textDecoration: "none" }}>
        <ChevronLeft size={16} /> Back
      </Link>

      <div className="detail-backdrop">
        {item.backdrop_url ? (
          <img src={item.backdrop_url} alt="" />
        ) : (
          <div style={{ width: "100%", height: "100%", background: "linear-gradient(135deg, rgba(0,164,220,0.1) 0%, rgba(170,92,195,0.05) 100%)" }} />
        )}
        <div className="detail-backdrop-overlay" />
      </div>

      <div className="detail-content">
        <div className="detail-poster">
          {item.poster_url ? (
            <img src={item.poster_url} alt={item.title} />
          ) : (
            <div style={{ aspectRatio: "2/3", display: "flex", alignItems: "center", justifyContent: "center", background: "rgba(255,255,255,0.04)" }}>
              <TypeIcon size={48} strokeWidth={1} style={{ opacity: 0.3 }} />
            </div>
          )}
        </div>

        <div className="detail-info">
          <h1 className="detail-title">{item.title}</h1>

          <div className="detail-meta">
            {item.year && (
              <span style={{ display: "flex", alignItems: "center", gap: "4px" }}>
                <Calendar size={14} /> {item.year}
              </span>
            )}
            {item.rating && (
              <span style={{ display: "flex", alignItems: "center", gap: "4px", color: "var(--jf-star)" }}>
                <Star size={14} fill="var(--jf-star)" /> {item.rating.toFixed(1)}
              </span>
            )}
            <span className={`badge ${status.className}`}>
              <StatusIcon size={12} /> {status.label}
            </span>
            {item.media_type && (
              <span className="badge">
                <TypeIcon size={12} /> {item.media_type}
              </span>
            )}
            {item.seasonCount && <span>{item.seasonCount} Seasons</span>}
            {item.episodeCount && <span>{item.episodeCount} Episodes</span>}
            {item.artistName && <span>{item.artistName}</span>}
            {item.authorName && <span>{item.authorName}</span>}
          </div>

          {item.genres && item.genres.length > 0 && (
            <div className="detail-genres">
              {item.genres.map((genre: string) => (
                <span key={genre} className="badge genre-badge">{genre}</span>
              ))}
            </div>
          )}

          {item.overview && (
            <p className="detail-overview">{item.overview}</p>
          )}

          <div style={{ marginTop: "24px" }}>
            {requested ? (
              <button className="btn btn-success" disabled style={{ background: "rgba(67,160,71,0.2)", color: "#66bb6a", border: "1px solid rgba(67,160,71,0.3)" }}>
                <CheckCircle2 size={16} /> Requested
              </button>
            ) : (
              <button className="btn btn-primary" onClick={handleRequest} disabled={requesting}>
                <Send size={16} />
                {requesting ? "Requesting..." : "Request"}
              </button>
            )}
          </div>
        </div>
      </div>

      {item.cast && item.cast.length > 0 && (
        <div style={{ marginTop: "48px" }}>
          <h2 className="section-title" style={{ marginBottom: "16px" }}>Cast</h2>
          <div className="detail-cast">
            {item.cast.map((person: CastMember, idx: number) => (
              <div key={idx} className="detail-cast-item">
                {person.image ? (
                  <img src={person.image} alt={person.name} />
                ) : (
                  <div style={{ width: "64px", height: "64px", borderRadius: "50%", margin: "0 auto 6px", background: "rgba(255,255,255,0.06)", display: "flex", alignItems: "center", justifyContent: "center", color: "var(--jf-text-secondary)", fontSize: "1.2rem", fontWeight: 600 }}>
                    {person.name?.charAt(0) || "?"}
                  </div>
                )}
                <div className="detail-cast-name">{person.name}</div>
                <div className="detail-cast-role">{person.role}</div>
              </div>
            ))}
          </div>
        </div>
      )}

      {similar.length > 0 && (
        <div style={{ marginTop: "48px" }}>
          <h2 className="section-title" style={{ marginBottom: "16px" }}>Similar Items</h2>
          <div className="detail-similar">
            {similar.map((sim: MediaItem) => (
              <Link key={sim.id} href={`/detail?id=${encodeURIComponent(sim.id)}`} className="detail-similar-item" style={{ textDecoration: "none", color: "inherit" }}>
                {sim.poster_url ? (
                  <img src={sim.poster_url} alt={sim.title} />
                ) : (
                  <div style={{ aspectRatio: "2/3", display: "flex", alignItems: "center", justifyContent: "center", background: "rgba(255,255,255,0.04)" }}>
                    <Film size={24} style={{ opacity: 0.2 }} />
                  </div>
                )}
                <div className="detail-similar-item-title">{sim.title}</div>
              </Link>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
