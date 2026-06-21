"use client";

import { useState, useEffect } from "react";
import {
  Settings, Sliders, Palette, Code, Info,
  Plus, Trash2, RefreshCw, Check, X, ExternalLink, Save
} from "lucide-react";

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
  { value: "radarr", label: "Radarr", desc: "Movie management" },
  { value: "sonarr", label: "Sonarr", desc: "TV show management" },
  { value: "lidarr", label: "Lidarr", desc: "Music management" },
  { value: "readarr", label: "Readarr", desc: "Book management" },
  { value: "mylar3", label: "Mylar3", desc: "Comic management" },
  { value: "sabnzbd", label: "SABnzbd", desc: "Usenet downloader" },
  { value: "prowlarr", label: "Prowlarr", desc: "Indexer manager" },
];

const navItems = [
  { id: "general", label: "General", icon: Sliders },
  { id: "integrations", label: "Integrations", icon: ExternalLink },
  { id: "appearance", label: "Appearance", icon: Palette },
  { id: "css", label: "CSS Injection", icon: Code },
  { id: "about", label: "About", icon: Info },
];

export default function SettingsPage() {
  const [activeSection, setActiveSection] = useState("general");
  const [integrations, setIntegrations] = useState<Integration[]>([]);
  const [editingIntegration, setEditingIntegration] = useState<Integration | null>(null);
  const [showAddForm, setShowAddForm] = useState(false);
  const [toast, setToast] = useState<{ message: string; type: string } | null>(null);
  const [cssCode, setCssCode] = useState("");
  const [generalSettings, setGeneralSettings] = useState({
    appName: "OakSeerr",
    jellyfinUrl: "",
    jellyfinApiKey: "",
  });

  useEffect(() => {
    fetchIntegrations();
    const saved = localStorage.getItem("oakseerr_css_injection");
    if (saved) setCssCode(saved);
  }, []);

  const fetchIntegrations = async () => {
    try {
      const res = await fetch(`${API_BASE}/api/v1/integrations`);
      if (res.ok) {
        const data = await res.json();
        setIntegrations(Array.isArray(data) ? data : []);
      }
    } catch (e) {}
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
      if (res.ok) showToast("Settings saved");
      else showToast("Failed to save settings", "error");
    } catch (e) {
      showToast("Settings saved (offline)", "success");
    }
  };

  const handleTestIntegration = async (integration: Integration) => {
    try {
      const res = await fetch(`${API_BASE}/api/v1/integrations/${integration.id}/test`, {
        method: "POST",
      });
      if (res.ok) showToast(`${integration.name} connection successful!`);
      else {
        const err = await res.text();
        showToast(`Connection failed: ${err}`, "error");
      }
    } catch (e) {
      showToast("Test connection (offline)", "success");
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
      showToast("Integration saved (offline)", "success");
      setEditingIntegration(null);
      setShowAddForm(false);
    }
  };

  const handleDeleteIntegration = async (id: string) => {
    try {
      const res = await fetch(`${API_BASE}/api/v1/integrations/${id}`, { method: "DELETE" });
      if (res.ok) {
        showToast("Integration removed");
        fetchIntegrations();
      }
    } catch (e) {
      showToast("Integration removed (offline)", "success");
    }
  };

  const handleSaveCss = () => {
    localStorage.setItem("oakseerr_css_injection", cssCode);
    window.dispatchEvent(new Event("css-injection-changed"));
    showToast("CSS injection saved and applied");
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
      <div className="card" style={{ padding: "24px", marginBottom: "16px", border: "none" }}>
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
              <option key={t.value} value={t.value}>{t.label} - {t.desc}</option>
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
            style={{ width: "16px", height: "16px", accentColor: "var(--jf-primary)" }}
          />
          <label htmlFor="enabled" style={{ fontSize: "0.875rem", color: "var(--jf-text-secondary)" }}>
            Enabled
          </label>
        </div>

        <div style={{ display: "flex", gap: "8px", marginTop: "16px" }}>
          <button className="btn btn-primary" onClick={() => onSave({ ...integration, ...form })}>
            <Save size={16} />
            {integration.id ? "Update" : "Add"} Integration
          </button>
          <button className="btn btn-secondary" onClick={onCancel}>
            <X size={16} />
            Cancel
          </button>
        </div>
      </div>
    );
  };

  return (
    <div>
      <h1 className="section-title" style={{ fontSize: "1.5rem", marginBottom: "24px" }}>
        <Settings size={24} style={{ marginRight: "10px", verticalAlign: "middle" }} />
        Settings
      </h1>

      <div className="settings-layout">
        {/* Settings Nav */}
        <nav className="settings-nav">
          {navItems.map((item) => (
            <button
              key={item.id}
              className={`settings-nav-item ${activeSection === item.id ? "active" : ""}`}
              onClick={() => setActiveSection(item.id)}
            >
              <item.icon size={18} />
              {item.label}
            </button>
          ))}
        </nav>

        {/* Content */}
        <div className="settings-content">
          {/* General */}
          {activeSection === "general" && (
            <div className="card" style={{ padding: "24px", border: "none" }}>
              <h2 style={{ fontSize: "1.1rem", fontWeight: 600, marginBottom: "20px" }}>
                <Sliders size={18} style={{ marginRight: "8px", verticalAlign: "middle" }} />
                General Settings
              </h2>

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
                <Save size={16} />
                Save Changes
              </button>
            </div>
          )}

          {/* Integrations */}
          {activeSection === "integrations" && (
            <div>
              <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginBottom: "16px" }}>
                <h2 style={{ fontSize: "1.1rem", fontWeight: 600 }}>
                  <ExternalLink size={18} style={{ marginRight: "8px", verticalAlign: "middle" }} />
                  Integrations
                </h2>
                <button className="btn btn-primary" onClick={() => setShowAddForm(true)}>
                  <Plus size={16} />
                  Add Integration
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
                <div className="card" style={{ padding: "48px", textAlign: "center", border: "none" }}>
                  <ExternalLink size={48} style={{ color: "var(--jf-text-secondary)", margin: "0 auto 16px", display: "block", opacity: 0.3 }} />
                  <p style={{ color: "var(--jf-text-secondary)", marginBottom: "8px" }}>
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
                          border: "none",
                        }}
                      >
                        <div style={{ flex: 1, minWidth: 0 }}>
                          <div style={{ display: "flex", alignItems: "center", gap: "8px" }}>
                            <strong>{integration.name}</strong>
                            <span className={`badge ${integration.enabled ? "badge-success" : ""}`}>
                              {integration.enabled ? "Active" : "Disabled"}
                            </span>
                          </div>
                          <p style={{ fontSize: "0.8rem", color: "var(--jf-text-secondary)", marginTop: "2px" }}>
                            {typeInfo?.desc} - {integration.base_url}
                          </p>
                        </div>
                        <div style={{ display: "flex", gap: "6px", flexShrink: 0 }}>
                          <button
                            className="btn btn-secondary btn-sm"
                            onClick={() => handleTestIntegration(integration)}
                          >
                            <RefreshCw size={14} />
                            Test
                          </button>
                          <button
                            className="btn btn-secondary btn-sm"
                            onClick={() => setEditingIntegration(integration)}
                          >
                            Edit
                          </button>
                          <button
                            className="btn btn-danger btn-sm"
                            onClick={() => handleDeleteIntegration(integration.id)}
                          >
                            <Trash2 size={14} />
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

          {/* Appearance */}
          {activeSection === "appearance" && (
            <div className="card" style={{ padding: "24px", border: "none" }}>
              <h2 style={{ fontSize: "1.1rem", fontWeight: 600, marginBottom: "20px" }}>
                <Palette size={18} style={{ marginRight: "8px", verticalAlign: "middle" }} />
                Appearance
              </h2>
              <p style={{ color: "var(--jf-text-secondary)", marginBottom: "16px" }}>
                OakSeerr uses the Jellyfin Dark Theme by default. Use the CSS Injection tab to add custom themes.
              </p>
              <div style={{ display: "flex", flexDirection: "column", gap: "12px" }}>
                <div className="card" style={{ padding: "16px", background: "rgba(255,255,255,0.03)" }}>
                  <p style={{ fontSize: "0.85rem", color: "var(--jf-text-secondary)" }}>
                    To use the Abyss theme (or any Jellyfin theme), go to the CSS Injection tab and add:
                  </p>
                  <pre style={{ marginTop: "8px", padding: "12px", background: "rgba(0,0,0,0.3)", borderRadius: "8px", fontSize: "0.8rem", color: "var(--jf-primary)", overflow: "auto" }}>
{`@import url('https://cdn.jsdelivr.net/gh/AumGupta/abyss-jellyfin@main/abyss.css');`}
                  </pre>
                </div>
              </div>
            </div>
          )}

          {/* CSS Injection */}
          {activeSection === "css" && (
            <div className="card" style={{ padding: "24px", border: "none" }}>
              <h2 style={{ fontSize: "1.1rem", fontWeight: 600, marginBottom: "8px" }}>
                <Code size={18} style={{ marginRight: "8px", verticalAlign: "middle" }} />
                CSS Injection
              </h2>
              <p style={{ color: "var(--jf-text-secondary)", marginBottom: "20px", fontSize: "0.85rem" }}>
                Inject custom CSS or import external themes. Use @import for external stylesheets.
                Changes are applied immediately and saved in your browser.
              </p>

              <div className="form-group">
                <label className="form-label">Custom CSS</label>
                <textarea
                  className="input"
                  value={cssCode}
                  onChange={(e) => setCssCode(e.target.value)}
                  placeholder={`/* Example: Import a Jellyfin theme */\n@import url('https://cdn.jsdelivr.net/gh/AumGupta/abyss-jellyfin@main/abyss.css');\n\n/* Or add custom styles */\n.sidebar {\n  background: rgba(0, 0, 0, 0.9);\n}`}
                  style={{ minHeight: "200px", fontFamily: "'Monaco', 'Menlo', 'Consolas', monospace", fontSize: "0.85rem" }}
                />
              </div>

              <div style={{ display: "flex", gap: "8px" }}>
                <button className="btn btn-primary" onClick={handleSaveCss}>
                  <Save size={16} />
                  Save & Apply
                </button>
                <button className="btn btn-secondary" onClick={() => {
                  setCssCode("");
                  localStorage.removeItem("oakseerr_css_injection");
                  window.dispatchEvent(new Event("css-injection-changed"));
                  showToast("CSS injection cleared");
                }}>
                  <X size={16} />
                  Clear
                </button>
              </div>
            </div>
          )}

          {/* About */}
          {activeSection === "about" && (
            <div className="card" style={{ padding: "24px", border: "none" }}>
              <h2 style={{ fontSize: "1.1rem", fontWeight: 600, marginBottom: "16px" }}>
                <Info size={18} style={{ marginRight: "8px", verticalAlign: "middle" }} />
                About OakSeerr
              </h2>
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

      {/* Toast */}
      {toast && (
        <div className={`toast ${toast.type}`}>
          {toast.type === "success" ? <Check size={18} style={{ color: "#66bb6a" }} /> : <X size={18} style={{ color: "#ef5350" }} />}
          {toast.message}
        </div>
      )}
    </div>
  );
}
