"use client";

import { useState } from "react";
import MediaCard from "@/components/MediaCard";

const shows = [
  { id: "1", title: "Severance", year: 2022, type: "tv", poster: null, status: "available" },
  { id: "2", title: "The Last of Us", year: 2023, type: "tv", poster: null, status: "available" },
  { id: "3", title: "Stranger Things", year: 2016, type: "tv", poster: null, status: "requested" },
  { id: "4", title: "House of the Dragon", year: 2022, type: "tv", poster: null, status: "available" },
  { id: "5", title: "The Bear", year: 2022, type: "tv", poster: null, status: "processing" },
  { id: "6", title: "Succession", year: 2018, type: "tv", poster: null, status: "available" },
];

export default function TVPage() {
  const [search, setSearch] = useState("");

  return (
    <div>
      <div className="section-header">
        <h1 className="section-title" style={{ fontSize: "1.5rem" }}>TV Shows</h1>
        <input
          className="input"
          placeholder="Search TV shows..."
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          style={{ maxWidth: "300px" }}
        />
      </div>

      <div className="media-grid">
        {shows
          .filter((s) => s.title.toLowerCase().includes(search.toLowerCase()))
          .map((show) => (
            <MediaCard key={show.id} item={show} />
          ))}
      </div>
    </div>
  );
}
