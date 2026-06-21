"use client";

import { useState, useEffect } from "react";
import { Search, Music } from "lucide-react";
import MediaCard from "@/components/MediaCard";

const API_BASE = process.env.NEXT_PUBLIC_API_URL || "";

export default function MusicPage() {
  const [search, setSearch] = useState("");
  const [items, setItems] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function fetchMusic() {
      setLoading(true);
      try {
        const url = search.trim()
          ? `${API_BASE}/api/v1/media/search?q=${encodeURIComponent(search)}&media_type=music`
          : `${API_BASE}/api/v1/media/trending?media_type=music`;
        const res = await fetch(url);
        if (res.ok) {
          const data = await res.json();
          setItems(Array.isArray(data) ? data : []);
        } else {
          setItems([]);
        }
      } catch (e) {
        setItems([]);
      }
      setLoading(false);
    }
    fetchMusic();
  }, [search]);

  return (
    <div>
      <div className="section-header">
        <h1 className="section-title">
          <Music size={22} style={{ marginRight: "8px", verticalAlign: "middle" }} />
          Music
        </h1>
        <div style={{ position: "relative", maxWidth: "300px", width: "100%" }}>
          <Search size={16} style={{ position: "absolute", left: "12px", top: "50%", transform: "translateY(-50%)", color: "var(--jf-text-secondary)", pointerEvents: "none" }} />
          <input
            className="input"
            placeholder="Search artists, albums..."
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            style={{ paddingLeft: "36px" }}
          />
        </div>
      </div>

      {loading ? (
        <div className="media-grid">
          {Array.from({ length: 8 }).map((_, i) => (
            <div key={i} className="skeleton" style={{ aspectRatio: "2/3", borderRadius: "12px" }} />
          ))}
        </div>
      ) : items.length === 0 ? (
        <div className="card" style={{ padding: "48px", textAlign: "center" }}>
          <Music size={48} style={{ color: "var(--jf-text-secondary)", margin: "0 auto 16px", display: "block", opacity: 0.3 }} />
          <p style={{ color: "var(--jf-text-secondary)" }}>
            {search.trim()
              ? `No music found for "${search}".`
              : "No music found. Add a Lidarr integration in Settings to see your music."}
          </p>
        </div>
      ) : (
        <div className="media-grid">
          {items.map((item: any) => (
            <MediaCard key={item.id} item={item} />
          ))}
        </div>
      )}
    </div>
  );
}
