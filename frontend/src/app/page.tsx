"use client";

import { useState, useEffect } from "react";
import { useRouter } from "next/navigation";
import { Search, TrendingUp, Film, Tv, Music, BookOpen, BookMarked, ListChecks } from "lucide-react";
import MediaCard from "@/components/MediaCard";
import type { MediaItem } from "@/types";

const API_BASE = process.env.NEXT_PUBLIC_API_URL || "";

const tabs = [
  { label: "Trending", icon: TrendingUp },
  { label: "Movies", icon: Film },
  { label: "TV", icon: Tv },
  { label: "Music", icon: Music },
  { label: "Books", icon: BookOpen },
  { label: "Comics", icon: BookMarked },
];

export default function Home() {
  const [activeTab, setActiveTab] = useState("Trending");
  const [searchQuery, setSearchQuery] = useState("");
  const [items, setItems] = useState<MediaItem[]>([]);
  const [recentRequests, setRecentRequests] = useState<MediaItem[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function fetchData() {
      setLoading(true);
      try {
        const mediaType = activeTab === "Trending" ? "" : activeTab.toLowerCase();
        let url: string;

        if (mediaType) {
          // Fetch library items for specific media type
          url = `${API_BASE}/api/v1/media/trending?media_type=${mediaType}`;
        } else {
          // Fetch all trending
          url = `${API_BASE}/api/v1/media/trending`;
        }

        const [mediaRes, requestsRes] = await Promise.all([
          fetch(url),
          fetch(`${API_BASE}/api/v1/requests`),
        ]);

        if (mediaRes.ok) {
          const data = await mediaRes.json();
          setItems(Array.isArray(data) ? data : []);
        } else {
          setItems([]);
        }

        if (requestsRes.ok) {
          const data = await requestsRes.json();
          setRecentRequests(Array.isArray(data) ? data : []);
        } else {
          setRecentRequests([]);
        }
      } catch {
        setItems([]);
        setRecentRequests([]);
      }
      setLoading(false);
    }
    fetchData();
  }, [activeTab]);

  const router = useRouter();

  const handleSearch = (e: React.FormEvent) => {
    e.preventDefault();
    if (searchQuery.trim()) {
      const target = activeTab === "Trending" ? "movies" : activeTab.toLowerCase();
      router.push(`/${target}?search=${encodeURIComponent(searchQuery)}`);
    }
  };

  return (
    <div>
      {/* Search Hero */}
      <div className="card" style={{ padding: "40px 32px", marginBottom: "32px", border: "none", background: "linear-gradient(135deg, rgba(0,164,220,0.12) 0%, rgba(170,92,195,0.08) 100%)" }}>
        <h1 style={{ fontSize: "1.8rem", fontWeight: 700, marginBottom: "8px" }}>
          Welcome to OakSeerr
        </h1>
        <p style={{ color: "var(--jf-text-secondary)", marginBottom: "24px", fontSize: "0.95rem" }}>
          Request movies, TV shows, music, books, and comics — all in one place.
        </p>
        <form onSubmit={handleSearch} style={{ display: "flex", gap: "12px", maxWidth: "600px" }}>
          <div style={{ flex: 1, position: "relative" }}>
            <Search size={18} style={{ position: "absolute", left: "14px", top: "50%", transform: "translateY(-50%)", color: "var(--jf-text-secondary)", pointerEvents: "none" }} />
            <input
              className="input"
              placeholder="Search for movies, TV shows, music, books..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              style={{ paddingLeft: "40px" }}
            />
          </div>
          <button type="submit" className="btn btn-primary">Search</button>
        </form>
      </div>

      {/* Tabs */}
      <div className="tabs">
        {tabs.map((tab) => (
          <button
            key={tab.label}
            className={`tab ${activeTab === tab.label ? "active" : ""}`}
            onClick={() => setActiveTab(tab.label)}
          >
            <tab.icon size={16} style={{ marginRight: "6px", verticalAlign: "middle" }} />
            {tab.label}
          </button>
        ))}
      </div>

      {/* Section Header */}
      <div className="section-header">
        <h2 className="section-title">
          {activeTab === "Trending" ? "Trending" : `Popular ${activeTab}`}
        </h2>
        {items.length > 6 && (
          <button className="btn btn-secondary btn-sm">View All</button>
        )}
      </div>

      {/* Media Grid */}
      {loading ? (
        <div className="media-grid">
          {Array.from({ length: 6 }).map((_, i) => (
            <div key={i} className="skeleton" style={{ aspectRatio: "2/3", borderRadius: "12px" }} />
          ))}
        </div>
      ) : items.length === 0 ? (
        <div className="card" style={{ padding: "48px", textAlign: "center" }}>
          <p style={{ color: "var(--jf-text-secondary)" }}>
            {activeTab === "Trending"
              ? "No trending media found. Configure your integrations in Settings to see your media."
              : `No ${activeTab.toLowerCase()} found. Add a ${activeTab === "Movies" ? "Radarr" : activeTab === "TV" ? "Sonarr" : activeTab === "Music" ? "Lidarr" : activeTab === "Books" ? "Readarr" : "Mylar3"} integration in Settings.`}
          </p>
        </div>
      ) : (
        <div className="media-grid">
          {items.map((item: MediaItem) => (
            <MediaCard key={item.id} item={item} />
          ))}
        </div>
      )}

      {/* Recent Requests */}
      <div style={{ marginTop: "48px" }}>
        <div className="section-header">
          <h2 className="section-title">
            <ListChecks size={20} style={{ marginRight: "8px", verticalAlign: "middle" }} />
            Recent Requests
          </h2>
          <button className="btn btn-secondary btn-sm" onClick={() => router.push("/requests")}>
            View All
          </button>
        </div>

        <div className="card" style={{ padding: "24px", border: "none" }}>
          {recentRequests.length === 0 ? (
            <p style={{ color: "var(--jf-text-secondary)", textAlign: "center", padding: "24px" }}>
              No recent requests. Start by searching for something above!
            </p>
          ) : (
            <div style={{ overflowX: "auto" }}>
              <table style={{ width: "100%", borderCollapse: "collapse" }}>
                <thead>
                  <tr style={{ borderBottom: "1px solid rgba(255,255,255,0.06)" }}>
                    <th style={{ padding: "12px 16px", textAlign: "left", color: "var(--jf-text-secondary)", fontWeight: 500, fontSize: "0.8rem" }}>Title</th>
                    <th style={{ padding: "12px 16px", textAlign: "left", color: "var(--jf-text-secondary)", fontWeight: 500, fontSize: "0.8rem" }}>Type</th>
                    <th style={{ padding: "12px 16px", textAlign: "left", color: "var(--jf-text-secondary)", fontWeight: 500, fontSize: "0.8rem" }}>Status</th>
                    <th style={{ padding: "12px 16px", textAlign: "left", color: "var(--jf-text-secondary)", fontWeight: 500, fontSize: "0.8rem" }}>Date</th>
                  </tr>
                </thead>
                <tbody>
                  {recentRequests.slice(0, 5).map((req: MediaItem) => (
                    <tr key={req.id} style={{ borderBottom: "1px solid rgba(255,255,255,0.04)" }}>
                      <td style={{ padding: "12px 16px", fontWeight: 500 }}>{req.title}</td>
                      <td style={{ padding: "12px 16px" }}>
                        <span className="badge">{req.media_type}</span>
                      </td>
                      <td style={{ padding: "12px 16px" }}>
                        <span className={`badge ${
                          req.status === "fulfilled" ? "badge-success" :
                          req.status === "approved" ? "badge-primary" :
                          req.status === "declined" ? "badge-error" : "badge-warning"
                        }`}>
                          {req.status}
                        </span>
                      </td>
                      <td style={{ padding: "12px 16px", color: "var(--jf-text-secondary)" }}>
                        {req.created_at ? new Date(req.created_at).toLocaleDateString() : "-"}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
