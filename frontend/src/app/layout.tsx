"use client";

import { usePathname } from "next/navigation";
import Link from "next/link";
import { useEffect, useState, useRef, useCallback } from "react";
import {
  Home, Film, Tv, Music, BookOpen, BookMarked,
  ListChecks, Settings, LogOut, Menu, X, Search
} from "lucide-react";
import "./globals.css";
import type { MediaItem } from "@/types";

const navSections = [
  {
    label: "Discover",
    items: [
      { label: "Home", href: "/", icon: Home },
      { label: "Movies", href: "/movies", icon: Film },
      { label: "TV Shows", href: "/tv", icon: Tv },
      { label: "Music", href: "/music", icon: Music },
      { label: "Books", href: "/books", icon: BookOpen },
      { label: "Comics", href: "/comics", icon: BookMarked },
    ],
  },
  {
    label: "Manage",
    items: [
      { label: "Requests", href: "/requests", icon: ListChecks },
      { label: "Settings", href: "/settings", icon: Settings },
    ],
  },
];

const API_BASE = process.env.NEXT_PUBLIC_API_URL || "";

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const pathname = usePathname();
  const [sidebarOpen, setSidebarOpen] = useState(true);
  const [searchQuery, setSearchQuery] = useState("");
  const [searchResults, setSearchResults] = useState<MediaItem[]>([]);
  const [searchOpen, setSearchOpen] = useState(false);
  const [searchLoading, setSearchLoading] = useState(false);
  const searchRef = useRef<HTMLDivElement>(null);
  const searchTimeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  // CSS Injection via @layer — wraps injected CSS in a layer so it can't break layout
  const applyCssInjection = useCallback((css: string) => {
    const old = document.getElementById("oakseerr-css-injection");
    if (old) old.remove();

    if (!css.trim()) return;

    // Wrap injected CSS in a @layer user-theme so it can't override layout
    const style = document.createElement("style");
    style.id = "oakseerr-css-injection";
    style.textContent = `@layer user-theme {\n${css}\n}`;
    document.head.appendChild(style);
  }, []);

  useEffect(() => {
    const saved = localStorage.getItem("oakseerr_css_injection");
    if (saved) {
      applyCssInjection(saved);
    }

    const handler = () => {
      const saved = localStorage.getItem("oakseerr_css_injection");
      applyCssInjection(saved || "");
    };
    window.addEventListener("css-injection-changed", handler);
    return () => window.removeEventListener("css-injection-changed", handler);
  }, [applyCssInjection]);

  // Search with debounce
  const handleSearchInput = useCallback((value: string) => {
    setSearchQuery(value);
    if (searchTimeoutRef.current) clearTimeout(searchTimeoutRef.current);

    if (!value.trim()) {
      setSearchResults([]);
      setSearchOpen(false);
      return;
    }

    searchTimeoutRef.current = setTimeout(async () => {
      setSearchLoading(true);
      try {
        const res = await fetch(`${API_BASE}/api/v1/media/search?q=${encodeURIComponent(value)}`);
        if (res.ok) {
          const data = await res.json();
          setSearchResults(Array.isArray(data) ? data.slice(0, 8) : []);
          setSearchOpen(true);
        }
      } catch (e) {
        setSearchResults([]);
      }
      setSearchLoading(false);
    }, 300);
  }, [API_BASE]);

  // Close search on click outside
  useEffect(() => {
    function handleClick(e: MouseEvent) {
      if (searchRef.current && !searchRef.current.contains(e.target as Node)) {
        setSearchOpen(false);
      }
    }
    document.addEventListener("mousedown", handleClick);
    return () => document.removeEventListener("mousedown", handleClick);
  }, []);

  const IconComponent = ({ icon: Icon, size = 20 }: { icon: React.ComponentType<{ size?: number; strokeWidth?: number }>; size?: number }) => (
    <Icon size={size} strokeWidth={1.5} />
  );

  return (
    <html lang="en">
      <body>
        <div className="app-layout">
          {sidebarOpen && (
            <div className="sidebar-overlay" onClick={() => setSidebarOpen(false)} />
          )}

          <aside className={`sidebar ${sidebarOpen ? "open" : ""}`}>
            <div className="sidebar-header">
              <div className="sidebar-logo">
                <div className="sidebar-logo-icon">
                  {/* Oak tree SVG logo in #00a4dc */}
                  <svg width="28" height="28" viewBox="0 0 28 28" fill="none" xmlns="http://www.w3.org/2000/svg">
                    {/* Trunk */}
                    <rect x="12" y="16" width="4" height="10" rx="1" fill="#00a4dc" opacity="0.6"/>
                    {/* Canopy layers */}
                    <ellipse cx="14" cy="8" rx="10" ry="8" fill="#00a4dc" opacity="0.3"/>
                    <ellipse cx="14" cy="7" rx="8" ry="6" fill="#00a4dc" opacity="0.5"/>
                    <ellipse cx="14" cy="6" rx="5" ry="4" fill="#00a4dc"/>
                    {/* Acorn */}
                    <ellipse cx="20" cy="20" rx="2.5" ry="3" fill="#00a4dc" opacity="0.7"/>
                    <rect x="19" y="17" width="2" height="1.5" rx="0.5" fill="#00a4dc" opacity="0.8"/>
                  </svg>
                </div>
                <span className="sidebar-logo-text">OakSeerr</span>
              </div>
              <button className="sidebar-close-btn" onClick={() => setSidebarOpen(false)}>
                <X size={20} />
              </button>
            </div>

            {/* Search bar in sidebar */}
            <div style={{ padding: "8px 12px" }} ref={searchRef}>
              <div style={{ position: "relative" }}>
                <Search size={16} style={{ position: "absolute", left: "10px", top: "50%", transform: "translateY(-50%)", color: "var(--jf-text-secondary)", pointerEvents: "none" }} />
                <input
                  className="input"
                  placeholder="Search..."
                  value={searchQuery}
                  onChange={(e) => handleSearchInput(e.target.value)}
                  onFocus={() => { if (searchResults.length > 0) setSearchOpen(true); }}
                  style={{ paddingLeft: "32px", paddingTop: "8px", paddingBottom: "8px", fontSize: "0.8rem" }}
                />
                {searchLoading && (
                  <div style={{ position: "absolute", right: "10px", top: "50%", transform: "translateY(-50%)" }}>
                    <div className="skeleton" style={{ width: "14px", height: "14px", borderRadius: "50%" }} />
                  </div>
                )}
                {/* Search dropdown */}
                {searchOpen && (
                  <div className="search-dropdown">
                    {searchResults.length === 0 ? (
                      <div className="search-dropdown-empty">No results found</div>
                    ) : (
                      searchResults.map((item: MediaItem) => (
                        <Link
                          key={item.id}
                          href={`/media/${item.id}`}
                          className="search-dropdown-item"
                          onClick={() => { setSearchOpen(false); setSearchQuery(""); }}
                        >
                          {item.poster_url ? (
                            <img src={item.poster_url} alt="" />
                          ) : (
                            <div style={{ width: "36px", height: "54px", borderRadius: "4px", background: "rgba(255,255,255,0.06)", flexShrink: 0 }} />
                          )}
                          <div className="search-dropdown-item-info">
                            <div className="search-dropdown-item-title">{item.title}</div>
                            <div className="search-dropdown-item-sub">
                              {item.year || ""} {item.media_type ? `· ${item.media_type}` : ""}
                            </div>
                          </div>
                        </Link>
                      ))
                    )}
                  </div>
                )}
              </div>
            </div>

            <nav className="sidebar-nav">
              {navSections.map((section) => (
                <div key={section.label} className="sidebar-section">
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
                        onClick={() => setSidebarOpen(false)}
                      >
                        <span className="sidebar-item-icon">
                          <IconComponent icon={item.icon} size={18} />
                        </span>
                        <span>{item.label}</span>
                      </Link>
                    );
                  })}
                </div>
              ))}
            </nav>

            <div className="sidebar-user">
              <div className="sidebar-user-avatar">D</div>
              <div className="sidebar-user-info">
                <div className="sidebar-user-name">Demo User</div>
                <div className="sidebar-user-role">Admin</div>
              </div>
              <button className="sidebar-user-logout" title="Logout">
                <LogOut size={16} />
              </button>
            </div>
          </aside>

          <main className="main-content">
            <div className="mobile-header">
              <button className="mobile-menu-btn" onClick={() => setSidebarOpen(true)}>
                <Menu size={24} />
              </button>
              <div className="mobile-logo">
                <svg width="24" height="24" viewBox="0 0 28 28" fill="none">
                  <rect x="12" y="16" width="4" height="10" rx="1" fill="#00a4dc" opacity="0.6"/>
                  <ellipse cx="14" cy="8" rx="10" ry="8" fill="#00a4dc" opacity="0.3"/>
                  <ellipse cx="14" cy="7" rx="8" ry="6" fill="#00a4dc" opacity="0.5"/>
                  <ellipse cx="14" cy="6" rx="5" ry="4" fill="#00a4dc"/>
                  <ellipse cx="20" cy="20" rx="2.5" ry="3" fill="#00a4dc" opacity="0.7"/>
                  <rect x="19" y="17" width="2" height="1.5" rx="0.5" fill="#00a4dc" opacity="0.8"/>
                </svg>
                <span>OakSeerr</span>
              </div>
            </div>
            {children}
          </main>
        </div>
      </body>
    </html>
  );
}
