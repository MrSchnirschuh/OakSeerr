import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import Navbar from "./Navbar";

vi.mock("next/navigation", () => ({
  usePathname: () => "/",
}));

vi.mock("next/link", () => ({
  default: ({ children, href }: { children: React.ReactNode; href: string }) => (
    <a href={href}>{children}</a>
  ),
}));

describe("Navbar", () => {
  it("renders navigation links", () => {
    render(<Navbar />);
    expect(screen.getByRole("navigation")).toBeInTheDocument();
    expect(screen.getByText(/movies/i)).toBeInTheDocument();
    expect(screen.getByText(/tv/i)).toBeInTheDocument();
    expect(screen.getByText(/music/i)).toBeInTheDocument();
    expect(screen.getByText(/books/i)).toBeInTheDocument();
    expect(screen.getByText(/comics/i)).toBeInTheDocument();
  });
});
