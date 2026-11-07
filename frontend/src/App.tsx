import { useState, type FormEvent } from "react";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { api, STATUSES, type Status } from "./api";

export default function App() {
  const queryClient = useQueryClient();
  const [filter, setFilter] = useState<Status | "">("");
  const [company, setCompany] = useState("");
  const [role, setRole] = useState("");

  const { data: apps = [], isLoading, error } = useQuery({
    queryKey: ["applications", filter],
    queryFn: () => api.list(filter || undefined),
  });

  const refresh = () => queryClient.invalidateQueries({ queryKey: ["applications"] });

  const createMutation = useMutation({
    mutationFn: api.create,
    onSuccess: () => {
      setCompany("");
      setRole("");
      refresh();
    },
  });

  const statusMutation = useMutation({
    mutationFn: ({ id, status }: { id: number; status: Status }) =>
      api.updateStatus(id, status),
    onSuccess: refresh,
  });

  const deleteMutation = useMutation({ mutationFn: api.remove, onSuccess: refresh });

  const handleSubmit = (e: FormEvent) => {
    e.preventDefault();
    createMutation.mutate({ company, role });
  };

  return (
    <main style={{ maxWidth: 720, margin: "2rem auto", fontFamily: "system-ui" }}>
      <h1>Job Application Tracker</h1>

      <form onSubmit={handleSubmit} style={{ display: "flex", gap: 8, marginBottom: 16 }}>
        <input placeholder="Company" value={company} onChange={(e) => setCompany(e.target.value)} />
        <input placeholder="Role" value={role} onChange={(e) => setRole(e.target.value)} />
        <button type="submit" disabled={createMutation.isPending}>Add</button>
      </form>
      {createMutation.error && <p style={{ color: "crimson" }}>{createMutation.error.message}</p>}

      <label>
        Filter:{" "}
        <select value={filter} onChange={(e) => setFilter(e.target.value as Status | "")}>
          <option value="">All</option>
          {STATUSES.map((s) => <option key={s}>{s}</option>)}
        </select>
      </label>

      {isLoading && <p>Loading…</p>}
      {error && <p style={{ color: "crimson" }}>{error.message}</p>}

      <ul style={{ listStyle: "none", padding: 0 }}>
        {apps.map((app) => (
          <li key={app.id} style={{ display: "flex", gap: 12, alignItems: "center", padding: "8px 0", borderBottom: "1px solid #eee" }}>
            <strong style={{ flex: 1 }}>{app.company} — {app.role}</strong>
            <select
              value={app.status}
              onChange={(e) => statusMutation.mutate({ id: app.id, status: e.target.value as Status })}
            >
              {STATUSES.map((s) => <option key={s}>{s}</option>)}
            </select>
            <button onClick={() => deleteMutation.mutate(app.id)}>Delete</button>
          </li>
        ))}
      </ul>
    </main>
  );
}