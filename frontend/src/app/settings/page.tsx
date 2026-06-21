"use client";

import { useState } from "react";

export default function SettingsPage() {
  const [activeSection, setActiveSection] = useState("general");

  return (
    <div>
      <h1 className="section-title" style={{ fontSize: "1.5rem", marginBottom: "24px" }}>Settings</h1>

      <div style={{ display: "flex", gap: "24px" }}>
        {/* Sidebar */}
        <div style={{ width: "200px", flexShrink: 0 }}>
          {["general", "integrations", "users", "notifications", "about"].map((section) => (
            <button
              key={section}
              onClick={() => setActiveSection(section)}
              style={{
                display: "block",
                width: "100%",
                padding: "10px 16px",
                textAlign: "left",
                background: activeSection === section ? "var(--jf-action-hover)" : "transparent",
                border: "none",
                borderRadius: "var(--jf-radius)",
                color: activeSection === section ? "var(--jf-primary)" : "var(--jf-text-secondary)",
                cursor: "pointer",
                fontSize: "0.875rem",
                fontFamily: "inherit",
                marginBottom: "4px",
              }}
            >
              {section.charAt(0).toUpperCase() + section.slice(1)}
            </button>
          ))}
        </div>

        {/* Content */}
        <div className="card" style={{ flex: 1, padding: "24px" }}>
          {activeSection === "general" && (
            <div>
              <h2 style={{ fontSize: "1.1rem", fontWeight: 600, marginBottom: "16px" }}>General Settings</h2>
              <div style={{ marginBottom: "16px" }}>
                <label style={{ display: "block", marginBottom: "6px", color: "var(--jf-text-secondary)", fontSize: "0.875rem" }}>Application Name</label>
                <input className="input" defaultValue="OakSeerr" style={{ maxWidth: "400px" }} />
              </div>
              <div style={{ marginBottom: "16px" }}>
                <label style={{ display: "block", marginBottom: "6px", color: "var(--jf-text-secondary)", fontSize: "0.875rem" }}>Jellyfin URL</label>
                <input className="input" placeholder="https://jellyfin.example.com" style={{ maxWidth: "400px" }} />
              </div>
              <div style={{ marginBottom: "16px" }}>
                <label style={{ display: "block", marginBottom: "6px", color: "var(--jf-text-secondary)", fontSize: "0.875rem" }}>Jellyfin API Key</label>
                <input className="input" type="password" placeholder="Enter API key" style={{ maxWidth: "400px" }} />
              </div>
              <button className="btn btn-primary" style={{ marginTop: "8px" }}>Save Changes</button>
            </div>
          )}

          {activeSection === "integrations" && (
            <div>
              <h2 style={{ fontSize: "1.1rem", fontWeight: 600, marginBottom: "16px" }}>Integrations</h2>
              <p style={{ color: "var(--jf-text-secondary)", marginBottom: "16px" }}>
                Connect your media management services.
              </p>
              {["Radarr", "Sonarr", "Lidarr", "Readarr", "Mylar3", "Sabnzbd", "Prowlarr"].map((svc) => (
                <div key={svc} className="card" style={{ padding: "16px", marginBottom: "8px", display: "flex", alignItems: "center", justifyContent: "space-between" }}>
                  <div>
                    <strong>{svc}</strong>
                    <p style={{ fontSize: "0.8rem", color: "var(--jf-text-secondary)", marginTop: "2px" }}>Not configured</p>
                  </div>
                  <button className="btn btn-secondary" style={{ padding: "6px 12px", fontSize: "0.8rem" }}>Configure</button>
                </div>
              ))}
            </div>
          )}

          {activeSection === "about" && (
            <div>
              <h2 style={{ fontSize: "1.1rem", fontWeight: 600, marginBottom: "16px" }}>About OakSeerr</h2>
              <p style={{ color: "var(--jf-text-secondary)", marginBottom: "8px" }}>Version: 0.1.0</p>
              <p style={{ color: "var(--jf-text-secondary)", marginBottom: "8px" }}>License: MIT</p>
              <p style={{ color: "var(--jf-text-secondary)" }}>
                All-in-one media request manager for Jellyfin. Movies, TV, Music, Books, Comics.
              </p>
            </div>
          )}

          {(activeSection === "users" || activeSection === "notifications") && (
            <div>
              <h2 style={{ fontSize: "1.1rem", fontWeight: 600, marginBottom: "16px" }}>
                {activeSection.charAt(0).toUpperCase() + activeSection.slice(1)}
              </h2>
              <p style={{ color: "var(--jf-text-secondary)" }}>Coming soon...</p>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
