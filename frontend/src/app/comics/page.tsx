"use client";

import { useState } from "react";
import MediaCard from "@/components/MediaCard";

const comics = [
  { id: "1", title: "Watchmen", year: 1986, type: "comic", poster: null, status: "available" },
  { id: "2", title: "The Sandman", year: 1989, type: "comic", poster: null, status: "requested" },
  { id: "3", title: "Batman: The Killing Joke", year: 1988, type: "comic", poster: null, status: "available" },
  { id: "4", title: "Saga", year: 2012, type: "comic", poster: null, status: "processing" },
  { id: "5", title: "Maus", year: 1980, type: "comic", poster: null, status: "available" },
  { id: "6", title: "V for Vendetta", year: 1982, type: "comic", poster: null, status: "available" },
];

export default function ComicsPage() {
  const [search, setSearch] = useState("");

  return (
    <div>
      <div className="section-header">
        <h1 className="section-title" style={{ fontSize: "1.5rem" }}>Comics</h1>
        <input
          className="input"
          placeholder="Search comics..."
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          style={{ maxWidth: "300px" }}
        />
      </div>

      <div className="media-grid">
        {comics
          .filter((c) => c.title.toLowerCase().includes(search.toLowerCase()))
          .map((comic) => (
            <MediaCard key={comic.id} item={comic} />
          ))}
      </div>
    </div>
  );
}
