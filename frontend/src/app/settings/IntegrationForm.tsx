"use client";

import { useState } from "react";
import { Save, X } from "lucide-react";

const integrationTypes = [
  { value: "radarr", label: "Radarr", desc: "Movie management" },
  { value: "sonarr", label: "Sonarr", desc: "TV show management" },
  { value: "lidarr", label: "Lidarr", desc: "Music management" },
  { value: "readarr", label: "Readarr", desc: "Book management" },
  { value: "mylar3", label: "Mylar3", desc: "Comic management" },
];

export interface IntegrationFormData {
  id?: string;
  name?: string;
  integration_type?: string;
  base_url?: string;
  api_key?: string;
  enabled?: boolean;
}

interface IntegrationFormProps {
  integration: Partial<IntegrationFormData>;
  onSave: (data: IntegrationFormData) => void;
  onCancel: () => void;
}

export default function IntegrationForm({ integration, onSave, onCancel }: IntegrationFormProps) {
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
}
