"use client";

import { useState } from "react";
import MediaCard from "@/components/MediaCard";

const books = [
  { id: "1", title: "Dune", year: 1965, type: "book", poster: null, status: "available" },
  { id: "2", title: "Neuromancer", year: 1984, type: "book", poster: null, status: "requested" },
  { id: "3", title: "The Hobbit", year: 1937, type: "book", poster: null, status: "available" },
  { id: "4", title: "Foundation", year: 1951, type: "book", poster: null, status: "available" },
  { id: "5", title: "Snow Crash", year: 1992, type: "book", poster: null, status: "processing" },
  { id: "6", title: "1984", year: 1949, type: "book", poster: null, status: "available" },
];

export default function BooksPage() {
  const [search, setSearch] = useState("");

  return (
    <div>
      <div className="section-header">
        <h1 className="section-title" style={{ fontSize: "1.5rem" }}>Books</h1>
        <input
          className="input"
          placeholder="Search books..."
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          style={{ maxWidth: "300px" }}
        />
      </div>

      <div className="media-grid">
        {books
          .filter((b) => b.title.toLowerCase().includes(search.toLowerCase()))
          .map((book) => (
            <MediaCard key={book.id} item={book} />
          ))}
      </div>
    </div>
  );
}
