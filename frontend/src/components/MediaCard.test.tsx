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

  it("renders poster image when poster_url is set", () => {
    render(<MediaCard item={{ ...baseItem, poster_url: "https://example.com/poster.jpg" }} />);
    const img = screen.getByAltText("Test Movie");
    expect(img).toBeInTheDocument();
    expect(img).toHaveAttribute("src", expect.stringContaining("example.com"));
  });

  it("renders placeholder when poster_url is null", () => {
    render(<MediaCard item={baseItem} />);
    expect(screen.getByTestId("media-type-icon")).toBeInTheDocument();
  });

  it("renders up to three genre badges", () => {
    render(<MediaCard item={{ ...baseItem, genres: ["Action", "Adventure", "Sci-Fi", "Drama"] }} />);
    expect(screen.getByText("Action")).toBeInTheDocument();
    expect(screen.getByText("Adventure")).toBeInTheDocument();
    expect(screen.getByText("Sci-Fi")).toBeInTheDocument();
    expect(screen.queryByText("Drama")).not.toBeInTheDocument();
  });
});
