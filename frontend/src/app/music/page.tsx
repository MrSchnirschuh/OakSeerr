"use client";

import { useState } from "react";
import MediaCard from "@/components/MediaCard";

const albums = [
  { id: "1", title: "The Dark Side of the Moon", year: 1973, type: "music", poster: null, status: "available" },
  { id: "2", title: "OK Computer", year: 1997, type: "music", poster: null, status: "available" },
  { id: "3", title: "Rumours", year: 1977, type: "music", poster: null, status: "requested" },
  { id: "4", title: "Thriller", year: 1982, type: "music", poster: null, status: "available" },
  { id: "5", title: "Nevermind", year: 1991, type: "music", poster: null, status: "processing" },
  { id: "6", title: "Abbey Road", year: 1969, type: "music", poster: null, status: "available" },
];

export default function MusicPage() {
  const [search, setSearch] = useState("");

  return (
    <div>
      <div className="section-header">
        <h1 className="section-title" style={{ fontSize: "1.5rem" }}>Music</h1>
        <input
          className="input"
          placeholder="Search artists, albums..."
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          style={{ maxWidth: "300px" }}
        />
      </div>

      <div className="media-grid">
        {albums
          .filter((a) => a.title.toLowerCase().includes(search.toLowerCase()))
          .map((album) => (
            <MediaCard key={album.id} item={album} />
          ))}
      </div>
    </div>
  );
}
