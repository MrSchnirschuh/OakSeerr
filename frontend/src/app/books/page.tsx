"use client";

import { useState, useEffect } from "react";
import { Search, BookOpen } from "lucide-react";
import MediaCard from "@/components/MediaCard";

const API_BASE = process.env.NEXT_PUBLIC_API_URL || "";

export default function BooksPage() {
  const [search, setSearch] = useState("");
  const [items, setItems] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function fetchBooks() {
      setLoading(true);
      try {
        const url = search.trim()
          ? `${API_BASE}/api/v1/media/search?q=${encodeURIComponent(search)}&media_type=book`
          : `${API_BASE}/api/v1/media/search?q=&media_type=book`;
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
    fetchBooks();
  }, [search]);

  return (
    <div>
      <div className="section-header">
        <h1 className="section-title">
          <BookOpen size={22} style={{ marginRight: "8px", verticalAlign: "middle" }} />
          Books
        </h1>
        <div style={{ position: "relative", maxWidth: "300px", width: "100%" }}>
          <Search size={16} style={{ position: "absolute", left: "12px", top: "50%", transform: "translateY(-50%)", color: "var(--jf-text-secondary)", pointerEvents: "none" }} />
          <input
            className="input"
            placeholder="Search books..."
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
          <BookOpen size={48} style={{ color: "var(--jf-text-secondary)", margin: "0 auto 16px", display: "block", opacity: 0.3 }} />
          <p style={{ color: "var(--jf-text-secondary)" }}>
            {search.trim()
              ? `No books found for "${search}".`
              : "No books found. Add a Readarr integration in Settings to see your books."}
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
