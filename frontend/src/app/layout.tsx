"use client";

import { usePathname } from "next/navigation";
import Link from "next/link";
import { useEffect, useState } from "react";
import {
  Home, Film, Tv, Music, BookOpen, BookMarked,
  ListChecks, Settings, LogOut, User,
  Menu, X, Palette, Code
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
  const [cssInjection, setCssInjection] = useState("");

  // Load CSS injection on mount
  useEffect(() => {
    const saved = localStorage.getItem("oakseerr_css_injection");
    if (saved) {
      setCssInjection(saved);
    }
  }, []);

  // Apply CSS injection
  useEffect(() => {
    let styleEl = document.getElementById("oakseerr-css-injection") as HTMLStyleElement | null;
    if (cssInjection) {
      if (!styleEl) {
        styleEl = document.createElement("style");
        styleEl.id = "oakseerr-css-injection";
        document.head.appendChild(styleEl);
      }
      styleEl.textContent = cssInjection;
    } else {
      if (styleEl) {
        styleEl.remove();
      }
    }
  }, [cssInjection]);

  // Listen for CSS injection changes from settings
  useEffect(() => {
    const handler = () => {
      const saved = localStorage.getItem("oakseerr_css_injection");
      setCssInjection(saved || "");
    };
    window.addEventListener("storage", handler);
    window.addEventListener("css-injection-changed", handler);
    return () => {
      window.removeEventListener("storage", handler);
      window.removeEventListener("css-injection-changed", handler);
    };
  }, []);

  const IconComponent = ({ icon: Icon, size = 20 }: { icon: any; size?: number }) => (
    <Icon size={size} strokeWidth={1.5} />
  );

  return (
    <html lang="en">
      <body>
        <div className="app-layout">
          {/* Mobile overlay */}
          {sidebarOpen && (
            <div className="sidebar-overlay" onClick={() => setSidebarOpen(false)} />
          )}

          {/* Sidebar */}
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

            {/* User area at bottom */}
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

          {/* Main content */}
          <main className="main-content">
            {/* Mobile hamburger */}
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
