"use client";

interface MediaItem {
  id: string;
  title: string;
  year?: number;
  type: string;
  poster: string | null;
  status: string;
}

export default function MediaCard({ item }: { item: MediaItem }) {
  const statusColors: Record<string, string> = {
    available: "badge-success",
    requested: "badge-primary",
    processing: "badge-warning",
    error: "badge-error",
  };

  const statusLabels: Record<string, string> = {
    available: "Available",
    requested: "Requested",
    processing: "Processing",
    error: "Error",
  };

  return (
    <div className="card" style={{ overflow: "hidden", cursor: "pointer" }}>
      {/* Poster placeholder */}
      <div
        style={{
          width: "100%",
          aspectRatio: "2/3",
          background: "linear-gradient(135deg, rgba(0,164,220,0.1) 0%, rgba(170,92,195,0.05) 100%)",
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
          fontSize: "2rem",
          color: "var(--jf-text-secondary)",
        }}
      >
        {item.type === "movie" ? "🎬" : item.type === "tv" ? "📺" : item.type === "music" ? "🎵" : item.type === "book" ? "📚" : "📖"}
      </div>

      {/* Info */}
      <div style={{ padding: "12px" }}>
        <h3
          style={{
            fontSize: "0.875rem",
            fontWeight: 600,
            marginBottom: "4px",
            overflow: "hidden",
            textOverflow: "ellipsis",
            whiteSpace: "nowrap",
          }}
        >
          {item.title}
        </h3>
        {item.year && (
          <p style={{ fontSize: "0.75rem", color: "var(--jf-text-secondary)", marginBottom: "8px" }}>
            {item.year}
          </p>
        )}
        <span className={`badge ${statusColors[item.status] || "badge"}`}>
          {statusLabels[item.status] || item.status}
        </span>
      </div>
    </div>
  );
}
