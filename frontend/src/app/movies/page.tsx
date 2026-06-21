"use client";

import { useState } from "react";
import MediaCard from "@/components/MediaCard";

const movies = [
  { id: "1", title: "Dune: Part Two", year: 2024, type: "movie", poster: null, status: "available" },
  { id: "2", title: "The Batman", year: 2022, type: "movie", poster: null, status: "requested" },
  { id: "3", title: "Interstellar", year: 2014, type: "movie", poster: null, status: "available" },
  { id: "4", title: "Blade Runner 2049", year: 2017, type: "movie", poster: null, status: "available" },
  { id: "5", title: "Everything Everywhere All at Once", year: 2022, type: "movie", poster: null, status: "processing" },
  { id: "6", title: "The Matrix", year: 1999, type: "movie", poster: null, status: "available" },
  { id: "7", title: "Inception", year: 2010, type: "movie", poster: null, status: "available" },
  { id: "8", title: "Parasite", year: 2019, type: "movie", poster: null, status: "requested" },
];

export default function MoviesPage() {
  const [search, setSearch] = useState("");

  return (
    <div>
      <div className="section-header">
        <h1 className="section-title" style={{ fontSize: "1.5rem" }}>Movies</h1>
        <input
          className="input"
          placeholder="Search movies..."
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          style={{ maxWidth: "300px" }}
        />
      </div>

      <div className="media-grid">
        {movies
          .filter((m) => m.title.toLowerCase().includes(search.toLowerCase()))
          .map((movie) => (
            <MediaCard key={movie.id} item={movie} />
          ))}
      </div>
    </div>
  );
}
