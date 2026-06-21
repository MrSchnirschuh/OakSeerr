"use client";

import { useState } from "react";
import MediaCard from "@/components/MediaCard";

// Mock data for the UI demo
const trendingMovies = [
  { id: "1", title: "Dune: Part Two", year: 2024, type: "movie", poster: null, status: "available" },
  { id: "2", title: "The Batman", year: 2022, type: "movie", poster: null, status: "requested" },
  { id: "3", title: "Interstellar", year: 2014, type: "movie", poster: null, status: "available" },
  { id: "4", title: "Blade Runner 2049", year: 2017, type: "movie", poster: null, status: "available" },
  { id: "5", title: "Everything Everywhere All at Once", year: 2022, type: "movie", poster: null, status: "processing" },
  { id: "6", title: "The Matrix", year: 1999, type: "movie", poster: null, status: "available" },
];

const tabs = ["Trending", "Movies", "TV", "Music", "Books", "Comics"];

export default function Home() {
  const [activeTab, setActiveTab] = useState("Trending");

  return (
    <div>
      {/* Hero / Search */}
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
        <input
          className="input"
          placeholder="Search for movies, TV shows, music, books..."
          style={{ maxWidth: "600px" }}
        />
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
        <h2 className="section-title">Trending</h2>
        <button className="btn btn-secondary">View All</button>
      </div>

      {/* Media Grid */}
      <div className="media-grid">
        {trendingMovies.map((movie) => (
          <MediaCard key={movie.id} item={movie} />
        ))}
      </div>

      {/* Recent Requests */}
      <div style={{ marginTop: "48px" }}>
        <div className="section-header">
          <h2 className="section-title">Recent Requests</h2>
          <button className="btn btn-secondary">View All</button>
        </div>

        <div className="card" style={{ padding: "24px" }}>
          <p style={{ color: "var(--jf-text-secondary)", textAlign: "center", padding: "32px" }}>
            No recent requests. Start by searching for something above!
          </p>
        </div>
      </div>
    </div>
  );
}
