"use client";

import { useState, useEffect } from "react";
import MediaCard from "@/components/MediaCard";

const API_BASE = process.env.NEXT_PUBLIC_API_URL || "";

const tabs = ["Trending", "Movies", "TV", "Music", "Books", "Comics"];

export default function Home() {
  const [activeTab, setActiveTab] = useState("Trending");
  const [searchQuery, setSearchQuery] = useState("");
  const [trending, setTrending] = useState<any[]>([]);
  const [recentRequests, setRecentRequests] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function fetchData() {
      try {
        const [trendingRes, requestsRes] = await Promise.all([
          fetch(`${API_BASE}/api/v1/media/trending`),
          fetch(`${API_BASE}/api/v1/requests`),
        ]);
        if (trendingRes.ok) {
          const data = await trendingRes.json();
          setTrending(data);
        }
        if (requestsRes.ok) {
          const data = await requestsRes.json();
          setRecentRequests(data);
        }
      } catch (e) {
        // Fallback mock data if API not available
        setTrending([
          { id: "1", title: "Dune: Part Two", year: 2024, type: "movie", poster: null, status: "available" },
          { id: "2", title: "The Batman", year: 2022, type: "movie", poster: null, status: "requested" },
          { id: "3", title: "Interstellar", year: 2014, type: "movie", poster: null, status: "available" },
          { id: "4", title: "Blade Runner 2049", year: 2017, type: "movie", poster: null, status: "available" },
          { id: "5", title: "Everything Everywhere All at Once", year: 2022, type: "movie", poster: null, status: "processing" },
          { id: "6", title: "The Matrix", year: 1999, type: "movie", poster: null, status: "available" },
        ]);
        setRecentRequests([]);
      }
      setLoading(false);
    }
    fetchData();
  }, []);

  const handleSearch = (e: React.FormEvent) => {
    e.preventDefault();
    if (searchQuery.trim()) {
      window.location.href = `/${activeTab.toLowerCase() === "trending" ? "movies" : activeTab.toLowerCase()}?search=${encodeURIComponent(searchQuery)}`;
    }
  };

  return (
    <div>
      {/* Search Hero */}
      <div
        style={{
          marginBottom: "32px",
          padding: "48px 32px",
          background: "linear-gradient(135deg, rgba(0,164,220,0.15) 0%, rgba(170,92,195,0.1) 100%)",
          borderRadius: "var(--jf-radius)",
          border: "1px solid var(--jf-divider)",
        }}
      >
        <h1 style={{ fontSize: "2rem", fontWeight: 700, marginBottom: "8px" }}>
          Welcome to OakSeerr
        </h1>
        <p style={{ color: "var(--jf-text-secondary)", marginBottom: "24px", fontSize: "1rem" }}>
          Request movies, TV shows, music, books, and comics — all in one place.
        </p>
        <form onSubmit={handleSearch}>
          <input
            className="input"
            placeholder="Search for movies, TV shows, music, books..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            style={{ maxWidth: "600px" }}
          />
        </form>
      </div>

      {/* Tabs */}
      <div className="tabs">
        {tabs.map((tab) => (
          <button
            key={tab}
            className={`tab ${activeTab === tab ? "active" : ""}`}
            onClick={() => setActiveTab(tab)}
          >
            {tab}
          </button>
        ))}
      </div>

      {/* Section Header */}
      <div className="section-header">
        <h2 className="section-title">
          {activeTab === "Trending" ? "Trending" : `Popular ${activeTab}`}
        </h2>
        <button className="btn btn-secondary">View All</button>
      </div>

      {/* Media Grid */}
      {loading ? (
        <div className="media-grid">
          {Array.from({ length: 6 }).map((_, i) => (
            <div key={i} className="skeleton" style={{ aspectRatio: "2/3", borderRadius: "var(--jf-radius)" }} />
          ))}
        </div>
      ) : (
        <div className="media-grid">
          {trending.map((item: any) => (
            <MediaCard key={item.id} item={item} />
          ))}
        </div>
      )}

      {/* Recent Requests */}
      <div style={{ marginTop: "48px" }}>
        <div className="section-header">
          <h2 className="section-title">Recent Requests</h2>
          <button className="btn btn-secondary" onClick={() => window.location.href = "/requests"}>
            View All
          </button>
        </div>

        <div className="card" style={{ padding: "24px" }}>
          {recentRequests.length === 0 ? (
            <p style={{ color: "var(--jf-text-secondary)", textAlign: "center", padding: "32px" }}>
              No recent requests. Start by searching for something above!
            </p>
          ) : (
            <table style={{ width: "100%", borderCollapse: "collapse" }}>
              <thead>
                <tr style={{ borderBottom: "1px solid var(--jf-divider)" }}>
                  <th style={{ padding: "12px 16px", textAlign: "left", color: "var(--jf-text-secondary)", fontWeight: 500, fontSize: "0.8rem" }}>Title</th>
                  <th style={{ padding: "12px 16px", textAlign: "left", color: "var(--jf-text-secondary)", fontWeight: 500, fontSize: "0.8rem" }}>Type</th>
                  <th style={{ padding: "12px 16px", textAlign: "left", color: "var(--jf-text-secondary)", fontWeight: 500, fontSize: "0.8rem" }}>Status</th>
                  <th style={{ padding: "12px 16px", textAlign: "left", color: "var(--jf-text-secondary)", fontWeight: 500, fontSize: "0.8rem" }}>Date</th>
                </tr>
              </thead>
              <tbody>
                {recentRequests.slice(0, 5).map((req: any) => (
                  <tr key={req.id} style={{ borderBottom: "1px solid var(--jf-divider)" }}>
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
                      {new Date(req.created_at).toLocaleDateString()}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          )}
        </div>
      </div>
    </div>
  );
}
