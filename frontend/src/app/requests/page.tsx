"use client";

import { useState } from "react";

interface RequestItem {
  id: string;
  title: string;
  type: string;
  requester: string;
  status: string;
  date: string;
}

const mockRequests: RequestItem[] = [
  { id: "1", title: "The Batman", type: "movie", requester: "You", status: "approved", date: "2024-01-15" },
  { id: "2", title: "OK Computer", type: "music", requester: "You", status: "pending", date: "2024-01-14" },
  { id: "3", title: "Neuromancer", type: "book", requester: "You", status: "pending", date: "2024-01-13" },
  { id: "4", title: "The Sandman", type: "comic", requester: "You", status: "fulfilled", date: "2024-01-12" },
];

export default function RequestsPage() {
  const [filter, setFilter] = useState("all");

  const filtered = filter === "all" ? mockRequests : mockRequests.filter((r) => r.status === filter);

  return (
    <div>
      <div className="section-header">
        <h1 className="section-title" style={{ fontSize: "1.5rem" }}>Requests</h1>
        <div style={{ display: "flex", gap: "8px" }}>
          {["all", "pending", "approved", "fulfilled", "declined"].map((f) => (
            <button
              key={f}
              className={`btn ${filter === f ? "btn-primary" : "btn-secondary"}`}
              onClick={() => setFilter(f)}
              style={{ padding: "6px 12px", fontSize: "0.8rem" }}
            >
              {f.charAt(0).toUpperCase() + f.slice(1)}
            </button>
          ))}
        </div>
      </div>

      <div className="card" style={{ overflow: "hidden" }}>
        <table style={{ width: "100%", borderCollapse: "collapse" }}>
          <thead>
            <tr style={{ borderBottom: "1px solid var(--jf-divider)" }}>
              <th style={{ padding: "12px 16px", textAlign: "left", color: "var(--jf-text-secondary)", fontWeight: 500, fontSize: "0.8rem" }}>Title</th>
              <th style={{ padding: "12px 16px", textAlign: "left", color: "var(--jf-text-secondary)", fontWeight: 500, fontSize: "0.8rem" }}>Type</th>
              <th style={{ padding: "12px 16px", textAlign: "left", color: "var(--jf-text-secondary)", fontWeight: 500, fontSize: "0.8rem" }}>Requester</th>
              <th style={{ padding: "12px 16px", textAlign: "left", color: "var(--jf-text-secondary)", fontWeight: 500, fontSize: "0.8rem" }}>Status</th>
              <th style={{ padding: "12px 16px", textAlign: "left", color: "var(--jf-text-secondary)", fontWeight: 500, fontSize: "0.8rem" }}>Date</th>
            </tr>
          </thead>
          <tbody>
            {filtered.map((req) => (
              <tr key={req.id} style={{ borderBottom: "1px solid var(--jf-divider)" }}>
                <td style={{ padding: "12px 16px", fontWeight: 500 }}>{req.title}</td>
                <td style={{ padding: "12px 16px", color: "var(--jf-text-secondary)" }}>
                  <span className="badge">{req.type}</span>
                </td>
                <td style={{ padding: "12px 16px", color: "var(--jf-text-secondary)" }}>{req.requester}</td>
                <td style={{ padding: "12px 16px" }}>
                  <span className={`badge ${
                    req.status === "fulfilled" ? "badge-success" :
                    req.status === "approved" ? "badge-primary" :
                    req.status === "declined" ? "badge-error" : "badge-warning"
                  }`}>
                    {req.status}
                  </span>
                </td>
                <td style={{ padding: "12px 16px", color: "var(--jf-text-secondary)" }}>{req.date}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
