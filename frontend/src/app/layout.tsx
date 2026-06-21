"use client";

import { usePathname } from "next/navigation";
import Link from "next/link";
import { useEffect, useState } from "react";
import {
  Home, Film, Tv, Music, BookOpen, BookMarked,
  ListChecks, Settings, LogOut, Menu, X, TrendingUp
} from "lucide-react";
import "./globals.css";

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

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const pathname = usePathname();
  const [sidebarOpen, setSidebarOpen] = useState(true);

  // CSS Injection via <link> statt <style> — damit @import funktioniert
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
  }, []);

  function applyCssInjection(css: string) {
    // Remove old injection
    const old = document.getElementById("oakseerr-css-injection");
    if (old) old.remove();
    const oldReset = document.getElementById("oakseerr-css-reset");
    if (oldReset) oldReset.remove();

    if (!css.trim()) return;

    // Inject a CSS reset layer FIRST that protects the base layout
    // This ensures the sidebar, main content, and app-layout always work
    const resetStyle = document.createElement("style");
    resetStyle.id = "oakseerr-css-reset";
    resetStyle.textContent = `
      /* Protect base layout from theme overrides */
      .app-layout, .sidebar, .main-content, .sidebar-nav, .sidebar-item,
      .sidebar-header, .sidebar-user, .media-card, .media-grid,
      .card, .btn, .input, .tabs, .tab, .badge, .section-header,
      .section-title, .settings-layout, .settings-nav, .settings-content,
      .form-group, .form-label, .form-hint, .toast, .detail-content,
      .detail-backdrop, .detail-info, .detail-poster, .detail-title,
      .detail-meta, .detail-genres, .detail-overview, .media-card-poster,
      .media-card-info, .media-card-title, .media-card-overlay,
      .media-card-status, .media-card-genres, .sidebar-logo,
      .sidebar-logo-text, .sidebar-section-label, .sidebar-item-icon,
      .sidebar-user-avatar, .sidebar-user-name, .sidebar-user-role,
      .sidebar-close-btn, .sidebar-user-logout, .mobile-header,
      .mobile-logo, .mobile-menu-btn, .sidebar-overlay,
      .skeleton, .progress-bar, .progress-bar-fill,
      .status-dot, .genre-badge, .badge-available, .badge-primary,
      .badge-success, .badge-warning, .badge-error,
      .btn-primary, .btn-secondary, .btn-danger, .btn-sm {
        all: revert-layer !important;
      }
      /* Ensure the app-layout is always flex */
      .app-layout {
        display: flex !important;
        height: 100vh !important;
      }
      .sidebar {
        width: var(--jf-sidebar-width, 240px) !important;
        flex-shrink: 0 !important;
      }
      .main-content {
        flex: 1 !important;
        overflow-y: auto !important;
      }
    `;
    document.head.appendChild(resetStyle);

    // Check if it's an @import URL — inject as <link> for proper CORS handling
    const importMatch = css.match(/@import\s+url\(['"]([^'"]+)['"]\)/);
    if (importMatch) {
      const link = document.createElement("link");
      link.id = "oakseerr-css-injection";
      link.rel = "stylesheet";
      link.href = importMatch[1];
      document.head.appendChild(link);
    } else {
      // Plain CSS — inject as <style>
      const style = document.createElement("style");
      style.id = "oakseerr-css-injection";
      style.textContent = css;
      document.head.appendChild(style);
    }
  }

  const IconComponent = ({ icon: Icon, size = 20 }: { icon: any; size?: number }) => (
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
                  <svg width="28" height="28" viewBox="0 0 28 28" fill="none">
                    <path d="M14 2L2 8v12l12 6 12-6V8L14 2z" fill="var(--jf-primary)" opacity="0.2"/>
                    <path d="M14 6L6 10v8l8 4 8-4v-8l-8-4z" fill="var(--jf-primary)" opacity="0.4"/>
                    <path d="M14 10l-4 2v4l4 2 4-2v-4l-4-2z" fill="var(--jf-primary)"/>
                  </svg>
                </div>
                <span className="sidebar-logo-text">OakSeerr</span>
              </div>
              <button className="sidebar-close-btn" onClick={() => setSidebarOpen(false)}>
                <X size={20} />
              </button>
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
                  <path d="M14 2L2 8v12l12 6 12-6V8L14 2z" fill="var(--jf-primary)" opacity="0.2"/>
                  <path d="M14 6L6 10v8l8 4 8-4v-8l-8-4z" fill="var(--jf-primary)" opacity="0.4"/>
                  <path d="M14 10l-4 2v4l4 2 4-2v-4l-4-2z" fill="var(--jf-primary)"/>
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
