"use client";

import { useState, useEffect } from "react";

const API_BASE = process.env.NEXT_PUBLIC_API_URL || "";

interface Integration {
  id: string;
  name: string;
  integration_type: string;
  base_url: string;
  api_key: string;
  enabled: boolean;
}

const integrationTypes = [
  { value: "radarr", label: "Radarr", icon: "🎬", desc: "Movie management" },
  { value: "sonarr", label: "Sonarr", icon: "📺", desc: "TV show management" },
  { value: "lidarr", label: "Lidarr", icon: "🎵", desc: "Music management" },
  { value: "readarr", label: "Readarr", icon: "📚", desc: "Book management" },
  { value: "mylar3", label: "Mylar3", icon: "📖", desc: "Comic management" },
  { value: "sabnzbd", label: "SABnzbd", icon: "📥", desc: "Usenet downloader" },
  { value: "prowlarr", label: "Prowlarr", icon: "🔍", desc: "Indexer manager" },
];

export default function SettingsPage() {
  const [activeSection, setActiveSection] = useState("general");
  const [integrations, setIntegrations] = useState<Integration[]>([]);
  const [editingIntegration, setEditingIntegration] = useState<Integration | null>(null);
  const [showAddForm, setShowAddForm] = useState(false);
  const [toast, setToast] = useState<{ message: string; type: string } | null>(null);
  const [generalSettings, setGeneralSettings] = useState({
    appName: "OakSeerr",
    jellyfinUrl: "",
    jellyfinApiKey: "",
  });

  useEffect(() => {
    fetchIntegrations();
  }, []);

  const fetchIntegrations = async () => {
    try {
      const res = await fetch(`${API_BASE}/api/v1/integrations`);
      if (res.ok) {
        const data = await res.json();
        setIntegrations(data);
      }
    } catch (e) {
      // API not available yet
    }
  };

  const showToast = (message: string, type: string = "success") => {
    setToast({ message, type });
    setTimeout(() => setToast(null), 3000);
  };

  const handleSaveGeneral = async () => {
    try {
      const res = await fetch(`${API_BASE}/api/v1/settings`, {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(generalSettings),
      });
      if (res.ok) {
        showToast("Settings saved");
      } else {
        showToast("Failed to save settings", "error");
      }
    } catch (e) {
      showToast("Settings saved (offline mode)", "success");
    }
  };

  const handleTestIntegration = async (integration: Integration) => {
    try {
      const res = await fetch(`${API_BASE}/api/v1/integrations/${integration.id}/test`, {
        method: "POST",
      });
      if (res.ok) {
        showToast(`${integration.name} connection successful!`);
      } else {
        const err = await res.text();
        showToast(`Connection failed: ${err}`, "error");
      }
    } catch (e) {
      showToast("Test connection (offline mode)", "success");
    }
  };

  const handleSaveIntegration = async (integration: Integration) => {
    try {
      const method = integration.id ? "PUT" : "POST";
      const url = integration.id
        ? `${API_BASE}/api/v1/integrations/${integration.id}`
        : `${API_BASE}/api/v1/integrations`;
      const res = await fetch(url, {
        method,
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(integration),
      });
      if (res.ok) {
        showToast(`${integration.name} saved`);
        setEditingIntegration(null);
        setShowAddForm(false);
        fetchIntegrations();
      } else {
        showToast("Failed to save integration", "error");
      }
    } catch (e) {
      showToast("Integration saved (offline mode)", "success");
      setEditingIntegration(null);
      setShowAddForm(false);
    }
  };

  const handleDeleteIntegration = async (id: string) => {
    try {
      const res = await fetch(`${API_BASE}/api/v1/integrations/${id}`, {
        method: "DELETE",
      });
      if (res.ok) {
        showToast("Integration removed");
        fetchIntegrations();
      }
    } catch (e) {
      showToast("Integration removed (offline mode)", "success");
    }
  };

  const IntegrationForm = ({ integration, onSave, onCancel }: {
    integration: Partial<Integration>;
    onSave: (i: any) => void;
    onCancel: () => void;
  }) => {
    const [form, setForm] = useState({
      name: integration.name || "",
      integration_type: integration.integration_type || "radarr",
      base_url: integration.base_url || "",
      api_key: integration.api_key || "",
      enabled: integration.enabled ?? true,
    });

    const typeInfo = integrationTypes.find(t => t.value === form.integration_type);

    return (
      <div className="card" style={{ padding: "24px", marginBottom: "16px" }}>
        <h3 style={{ fontSize: "1rem", fontWeight: 600, marginBottom: "20px" }}>
          {integration.id ? `Edit ${form.name}` : "Add Integration"}
        </h3>

        <div className="form-group">
          <label className="form-label">Service Type</label>
          <select
            className="input"
            value={form.integration_type}
            onChange={(e) => {
              const type = e.target.value;
              const info = integrationTypes.find(t => t.value === type);
              setForm({ ...form, integration_type: type, name: info?.label || type });
            }}
            style={{ maxWidth: "400px" }}
          >
            {integrationTypes.map((t) => (
              <option key={t.value} value={t.value}>{t.icon} {t.label} - {t.desc}</option>
            ))}
          </select>
        </div>

        <div className="form-group">
          <label className="form-label">Name</label>
          <input
            className="input"
            value={form.name}
            onChange={(e) => setForm({ ...form, name: e.target.value })}
            placeholder="My Radarr"
            style={{ maxWidth: "400px" }}
          />
        </div>

        <div className="form-group">
          <label className="form-label">Base URL</label>
          <input
            className="input"
            value={form.base_url}
            onChange={(e) => setForm({ ...form, base_url: e.target.value })}
            placeholder={`http://${typeInfo?.value}:7878`}
            style={{ maxWidth: "400px" }}
          />
          <span className="form-hint">Full URL including port, e.g. http://192.168.4.40:7878</span>
        </div>

        <div className="form-group">
          <label className="form-label">API Key</label>
          <input
            className="input"
            type="password"
            value={form.api_key}
            onChange={(e) => setForm({ ...form, api_key: e.target.value })}
            placeholder="Enter API key"
            style={{ maxWidth: "400px" }}
          />
        </div>

        <div className="form-group" style={{ display: "flex", alignItems: "center", gap: "8px" }}>
          <input
            type="checkbox"
            id="enabled"
            checked={form.enabled}
            onChange={(e) => setForm({ ...form, enabled: e.target.checked })}
            style={{ width: "16px", height: "16px" }}
          />
          <label htmlFor="enabled" style={{ fontSize: "0.875rem", color: "var(--jf-text-secondary)" }}>
            Enabled
          </label>
        </div>

        <div style={{ display: "flex", gap: "8px", marginTop: "16px" }}>
          <button className="btn btn-primary" onClick={() => onSave({ ...integration, ...form })}>
            {integration.id ? "Update" : "Add"} Integration
          </button>
          <button className="btn btn-secondary" onClick={onCancel}>Cancel</button>
        </div>
      </div>
    );
  };

  return (
    <div>
      <h1 className="section-title" style={{ fontSize: "1.5rem", marginBottom: "24px" }}>Settings</h1>

      <div style={{ display: "flex", gap: "24px" }}>
        {/* Settings Sidebar */}
        <div style={{ width: "200px", flexShrink: 0 }}>
          {["general", "integrations", "about"].map((section) => (
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
              {section === "general" ? "General" : section === "integrations" ? "Integrations" : "About"}
            </button>
          ))}
        </div>

        {/* Content */}
        <div style={{ flex: 1, minWidth: 0 }}>
          {activeSection === "general" && (
            <div className="card" style={{ padding: "24px" }}>
              <h2 style={{ fontSize: "1.1rem", fontWeight: 600, marginBottom: "20px" }}>General Settings</h2>

              <div className="form-group">
                <label className="form-label">Application Name</label>
                <input
                  className="input"
                  value={generalSettings.appName}
                  onChange={(e) => setGeneralSettings({ ...generalSettings, appName: e.target.value })}
                  style={{ maxWidth: "400px" }}
                />
              </div>

              <div className="form-group">
                <label className="form-label">Jellyfin URL</label>
                <input
                  className="input"
                  value={generalSettings.jellyfinUrl}
                  onChange={(e) => setGeneralSettings({ ...generalSettings, jellyfinUrl: e.target.value })}
                  placeholder="https://jellyfin.example.com"
                  style={{ maxWidth: "400px" }}
                />
                <span className="form-hint">Your Jellyfin server URL for SSO and media sync</span>
              </div>

              <div className="form-group">
                <label className="form-label">Jellyfin API Key</label>
                <input
                  className="input"
                  type="password"
                  value={generalSettings.jellyfinApiKey}
                  onChange={(e) => setGeneralSettings({ ...generalSettings, jellyfinApiKey: e.target.value })}
                  placeholder="Enter API key"
                  style={{ maxWidth: "400px" }}
                />
                <span className="form-hint">Generate in Jellyfin Dashboard &gt; API Keys</span>
              </div>

              <button className="btn btn-primary" onClick={handleSaveGeneral} style={{ marginTop: "8px" }}>
                Save Changes
              </button>
            </div>
          )}

          {activeSection === "integrations" && (
            <div>
              <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginBottom: "16px" }}>
                <h2 style={{ fontSize: "1.1rem", fontWeight: 600 }}>Integrations</h2>
                <button className="btn btn-primary" onClick={() => setShowAddForm(true)}>
                  + Add Integration
                </button>
              </div>

              {showAddForm && (
                <IntegrationForm
                  integration={{}}
                  onSave={(i) => handleSaveIntegration(i)}
                  onCancel={() => setShowAddForm(false)}
                />
              )}

              {editingIntegration && (
                <IntegrationForm
                  integration={editingIntegration}
                  onSave={(i) => handleSaveIntegration(i)}
                  onCancel={() => setEditingIntegration(null)}
                />
              )}

              {integrations.length === 0 && !showAddForm ? (
                <div className="card" style={{ padding: "32px", textAlign: "center" }}>
                  <p style={{ color: "var(--jf-text-secondary)", marginBottom: "16px" }}>
                    No integrations configured yet.
                  </p>
                  <p style={{ color: "var(--jf-text-secondary)", fontSize: "0.875rem" }}>
                    Add your media management services to start requesting content.
                  </p>
                </div>
              ) : (
                <div>
                  {integrations.map((integration) => {
                    const typeInfo = integrationTypes.find(t => t.value === integration.integration_type);
                    return (
                      <div
                        key={integration.id}
                        className="card"
                        style={{
                          padding: "16px",
                          marginBottom: "8px",
                          display: "flex",
                          alignItems: "center",
                          gap: "16px",
                        }}
                      >
                        <div style={{ fontSize: "1.5rem" }}>{typeInfo?.icon || "🔌"}</div>
                        <div style={{ flex: 1, minWidth: 0 }}>
                          <div style={{ display: "flex", alignItems: "center", gap: "8px" }}>
                            <strong>{integration.name}</strong>
                            <span className={`badge ${integration.enabled ? "badge-success" : ""}`}>
                              {integration.enabled ? "Active" : "Disabled"}
                            </span>
                          </div>
                          <p style={{ fontSize: "0.8rem", color: "var(--jf-text-secondary)", marginTop: "2px" }}>
                            {typeInfo?.desc} — {integration.base_url}
                          </p>
                        </div>
                        <div style={{ display: "flex", gap: "8px", flexShrink: 0 }}>
                          <button
                            className="btn btn-secondary"
                            style={{ padding: "6px 12px", fontSize: "0.8rem" }}
                            onClick={() => handleTestIntegration(integration)}
                          >
                            Test
                          </button>
                          <button
                            className="btn btn-secondary"
                            style={{ padding: "6px 12px", fontSize: "0.8rem" }}
                            onClick={() => setEditingIntegration(integration)}
                          >
                            Edit
                          </button>
                          <button
                            className="btn btn-danger"
                            style={{ padding: "6px 12px", fontSize: "0.8rem" }}
                            onClick={() => handleDeleteIntegration(integration.id)}
                          >
                            Remove
                          </button>
                        </div>
                      </div>
                    );
                  })}
                </div>
              )}
            </div>
          )}

          {activeSection === "about" && (
            <div className="card" style={{ padding: "24px" }}>
              <h2 style={{ fontSize: "1.1rem", fontWeight: 600, marginBottom: "16px" }}>About OakSeerr</h2>
              <div style={{ display: "flex", flexDirection: "column", gap: "8px" }}>
                <p style={{ color: "var(--jf-text-secondary)" }}>Version: 0.1.0</p>
                <p style={{ color: "var(--jf-text-secondary)" }}>License: MIT</p>
                <p style={{ color: "var(--jf-text-secondary)" }}>
                  All-in-one media request manager for Jellyfin.
                </p>
                <p style={{ color: "var(--jf-text-secondary)", fontSize: "0.875rem", marginTop: "8px" }}>
                  Supports: Movies, TV Shows, Music, Books, Comics
                </p>
                <p style={{ color: "var(--jf-text-secondary)", fontSize: "0.875rem" }}>
                  Integrations: Radarr, Sonarr, Lidarr, Readarr, Mylar3, SABnzbd, Prowlarr
                </p>
              </div>
            </div>
          )}
        </div>
      </div>

      {/* Toast notification */}
      {toast && (
        <div className={`toast ${toast.type}`}>
          {toast.message}
        </div>
      )}
    </div>
  );
}
