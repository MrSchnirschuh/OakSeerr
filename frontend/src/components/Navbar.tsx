"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";

const navItems = [
  { label: "Home", href: "/", icon: "home" },
  { label: "Movies", href: "/movies", icon: "movie" },
  { label: "TV", href: "/tv", icon: "tv" },
  { label: "Music", href: "/music", icon: "music_note" },
  { label: "Books", href: "/books", icon: "book" },
  { label: "Comics", href: "/comics", icon: "comic_bubble" },
  { label: "Requests", href: "/requests", icon: "add_circle" },
  { label: "Settings", href: "/settings", icon: "settings" },
];

export default function Navbar() {
  const pathname = usePathname();

  return (
    <nav className="navbar">
      <Link
        href="/"
        style={{
          fontSize: "1.25rem",
          fontWeight: 700,
          color: "var(--jf-primary)",
          textDecoration: "none",
          marginRight: "16px",
        }}
      >
        OakSeerr
      </Link>

      {navItems.map((item) => (
        <Link
          key={item.href}
          href={item.href}
          className={`nav-link ${pathname === item.href ? "active" : ""}`}
        >
          {item.label}
        </Link>
      ))}

      <div style={{ marginLeft: "auto", display: "flex", alignItems: "center", gap: "12px" }}>
        <div
          className="badge"
          style={{ cursor: "pointer" }}
        >
          Login
        </div>
      </div>
    </nav>
  );
}
