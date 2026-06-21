"use client";

import { usePathname } from "next/navigation";
import Link from "next/link";
import "./globals.css";

const navSections = [
  {
    label: "Discover",
    items: [
      { label: "Home", href: "/", icon: "🏠" },
      { label: "Movies", href: "/movies", icon: "🎬" },
      { label: "TV Shows", href: "/tv", icon: "📺" },
      { label: "Music", href: "/music", icon: "🎵" },
      { label: "Books", href: "/books", icon: "📚" },
      { label: "Comics", href: "/comics", icon: "📖" },
    ],
  },
  {
    label: "Manage",
    items: [
      { label: "Requests", href: "/requests", icon: "📋" },
      { label: "Settings", href: "/settings", icon: "⚙️" },
    ],
  },
];

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const pathname = usePathname();

  return (
    <html lang="en">
      <body>
        <div className="app-layout">
          {/* Sidebar */}
          <aside className="sidebar">
            <div className="sidebar-logo">
              <span style={{ fontSize: "1.5rem" }}>🌳</span>
              OakSeerr
            </div>

            <nav className="sidebar-nav">
              {navSections.map((section) => (
                <div key={section.label}>
                  <div className="sidebar-section-label">{section.label}</div>
                  {section.items.map((item) => {
                    const isActive =
                      item.href === "/"
                        ? pathname === "/"
                        : pathname.startsWith(item.href);
                    return (
                      <Link
                        key={item.href}
                        href={item.href}
                        className={`sidebar-item ${isActive ? "active" : ""}`}
                      >
                        <span className="sidebar-item-icon">{item.icon}</span>
                        {item.label}
                      </Link>
                    );
                  })}
                </div>
              ))}
            </nav>

            {/* User area at bottom */}
            <div
              style={{
                padding: "12px",
                borderTop: "1px solid var(--jf-divider)",
                display: "flex",
                alignItems: "center",
                gap: "10px",
              }}
            >
              <div
                style={{
                  width: "32px",
                  height: "32px",
                  borderRadius: "50%",
                  background: "var(--jf-primary)",
                  display: "flex",
                  alignItems: "center",
                  justifyContent: "center",
                  fontSize: "0.8rem",
                  fontWeight: 700,
                  color: "#000",
                  flexShrink: 0,
                }}
              >
                D
              </div>
              <div style={{ flex: 1, minWidth: 0 }}>
                <div
                  style={{
                    fontSize: "0.8rem",
                    fontWeight: 600,
                    overflow: "hidden",
                    textOverflow: "ellipsis",
                    whiteSpace: "nowrap",
                  }}
                >
                  Demo User
                </div>
                <div
                  style={{
                    fontSize: "0.7rem",
                    color: "var(--jf-text-secondary)",
                  }}
                >
                  Admin
                </div>
              </div>
            </div>
          </aside>

          {/* Main content */}
          <main className="main-content">{children}</main>
        </div>
      </body>
    </html>
  );
}
