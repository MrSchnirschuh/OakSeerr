"use client";

import { useState, useEffect } from "react";
import { ListChecks, CheckCircle2, Clock, XCircle, AlertCircle, Filter } from "lucide-react";
import type { MediaItem } from "@/types";

const API_BASE = process.env.NEXT_PUBLIC_API_URL || "";

const statusIcons: Record<string, React.ComponentType<{ size?: number }>> = {
  pending: Clock,
  approved: CheckCircle2,
  fulfilled: CheckCircle2,
  declined: XCircle,
  processing: AlertCircle,
};

const statusColors: Record<string, string> = {
  pending: "badge-warning",
  approved: "badge-primary",
  fulfilled: "badge-success",
  declined: "badge-error",
  processing: "badge-warning",
};

export default function RequestsPage() {
  const [requests, setRequests] = useState<MediaItem[]>([]);
  const [filter, setFilter] = useState("all");
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function fetchRequests() {
      setLoading(true);
      try {
        const res = await fetch(`${API_BASE}/api/v1/requests`);
        if (res.ok) {
          const data = await res.json();
          setRequests(Array.isArray(data) ? data : []);
        } else {
          setRequests([]);
        }
      } catch {
        setRequests([]);
      }
      setLoading(false);
    }
    fetchRequests();
  }, []);

  const filtered = filter === "all" ? requests : requests.filter((r) => r.status === filter);

  return (
    <div>
      <div className="section-header">
        <h1 className="section-title">
          <ListChecks size={22} style={{ marginRight: "8px", verticalAlign: "middle" }} />
          Requests
        </h1>
        <div style={{ display: "flex", gap: "6px", flexWrap: "wrap" }}>
          {["all", "pending", "approved", "fulfilled", "declined"].map((f) => (
            <button
              key={f}
              className={`btn btn-sm ${filter === f ? "btn-primary" : "btn-secondary"}`}
              onClick={() => setFilter(f)}
            >
              {f === "all" ? <Filter size={14} /> : null}
              {f.charAt(0).toUpperCase() + f.slice(1)}
            </button>
          ))}
        </div>
      </div>

      {loading ? (
        <div className="card" style={{ padding: "24px" }}>
          {Array.from({ length: 4 }).map((_, i) => (
            <div key={i} className="skeleton" style={{ height: "48px", marginBottom: "8px", borderRadius: "8px" }} />
          ))}
        </div>
      ) : filtered.length === 0 ? (
        <div className="card" style={{ padding: "48px", textAlign: "center" }}>
          <ListChecks size={48} style={{ color: "var(--jf-text-secondary)", margin: "0 auto 16px", display: "block", opacity: 0.3 }} />
          <p style={{ color: "var(--jf-text-secondary)" }}>
            {filter === "all"
              ? "No requests yet. Search for something and request it!"
              : `No ${filter} requests.`}
          </p>
        </div>
      ) : (
        <div className="card" style={{ overflow: "hidden", border: "none" }}>
          <div style={{ overflowX: "auto" }}>
            <table style={{ width: "100%", borderCollapse: "collapse" }}>
              <thead>
                <tr style={{ borderBottom: "1px solid rgba(255,255,255,0.06)" }}>
                  <th style={{ padding: "14px 16px", textAlign: "left", color: "var(--jf-text-secondary)", fontWeight: 500, fontSize: "0.8rem" }}>Title</th>
                  <th style={{ padding: "14px 16px", textAlign: "left", color: "var(--jf-text-secondary)", fontWeight: 500, fontSize: "0.8rem" }}>Type</th>
                  <th style={{ padding: "14px 16px", textAlign: "left", color: "var(--jf-text-secondary)", fontWeight: 500, fontSize: "0.8rem" }}>Requester</th>
                  <th style={{ padding: "14px 16px", textAlign: "left", color: "var(--jf-text-secondary)", fontWeight: 500, fontSize: "0.8rem" }}>Status</th>
                  <th style={{ padding: "14px 16px", textAlign: "left", color: "var(--jf-text-secondary)", fontWeight: 500, fontSize: "0.8rem" }}>Date</th>
                </tr>
              </thead>
              <tbody>
                {filtered.map((req: MediaItem) => {
                  const StatusIcon = statusIcons[req.status] || Clock;
                  return (
                    <tr key={req.id} style={{ borderBottom: "1px solid rgba(255,255,255,0.04)", transition: "background 0.15s" }}
                        onMouseEnter={(e) => (e.currentTarget.style.background = "rgba(255,255,255,0.03)")}
                        onMouseLeave={(e) => (e.currentTarget.style.background = "transparent")}>
                      <td style={{ padding: "14px 16px", fontWeight: 500 }}>{req.title}</td>
                      <td style={{ padding: "14px 16px" }}>
                        <span className="badge">{req.media_type}</span>
                      </td>
                      <td style={{ padding: "14px 16px", color: "var(--jf-text-secondary)" }}>{req.requester || "You"}</td>
                      <td style={{ padding: "14px 16px" }}>
                        <span className={`badge ${statusColors[req.status] || "badge"}`}>
                          <StatusIcon size={12} />
                          {req.status}
                        </span>
                      </td>
                      <td style={{ padding: "14px 16px", color: "var(--jf-text-secondary)" }}>
                        {req.created_at ? new Date(req.created_at).toLocaleDateString() : "-"}
                      </td>
                    </tr>
                  );
                })}
              </tbody>
            </table>
          </div>
        </div>
      )}
    </div>
  );
}
