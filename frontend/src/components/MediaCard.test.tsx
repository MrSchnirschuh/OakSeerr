import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import MediaCard from "./MediaCard";

const baseItem = {
  id: "tmdb-123",
  title: "Test Movie",
  year: 2024,
  media_type: "movie",
  poster_url: null,
  status: "available",
  rating: 8.2,
  genres: ["Sci-Fi", "Action"],
  overview: "A test overview that is long enough to render.",
};

describe("MediaCard", () => {
  it("renders title and status badge", () => {
    render(<MediaCard item={baseItem} />);
    expect(screen.getByText("Test Movie")).toBeInTheDocument();
    expect(screen.getByText("Available")).toBeInTheDocument();
  });

  it("renders year and genres", () => {
    render(<MediaCard item={baseItem} />);
    expect(screen.getByText("2024")).toBeInTheDocument();
    expect(screen.getByText("Sci-Fi")).toBeInTheDocument();
  });
});
