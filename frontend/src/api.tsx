export type Status = "Applied" | "Interview" | "Offer" | "Rejected";
export const STATUSES: Status[] = ["Applied", "Interview", "Offer", "Rejected"];

export interface Application {
  id: number;
  company: string;
  role: string;
  status: Status;
  job_url: string | null;
  notes: string | null;
  applied_on: string;
  updated_at: string;
}

export interface CreateApplication {
  company: string;
  role: string;
  status?: Status;
  job_url?: string;
  notes?: string;
}

const API_URL = "http://localhost:3000";

async function request<T>(path: string, init?: RequestInit): Promise<T> {
  const res = await fetch(`${API_URL}${path}`, {
    headers: { "Content-Type": "application/json" },
    ...init,
  });
  if (!res.ok) {
    const body = await res.json().catch(() => ({}));
    throw new Error(body.error ?? `Request failed (${res.status})`);
  }
  return res.status === 204 ? (undefined as T) : res.json();
}

export const api = {
  list: (status?: Status) =>
    request<Application[]>(`/applications${status ? `?status=${status}` : ""}`),
  create: (data: CreateApplication) =>
    request<Application>("/applications", { method: "POST", body: JSON.stringify(data) }),
  updateStatus: (id: number, status: Status) =>
    request<Application>(`/applications/${id}/status`, {
      method: "PATCH",
      body: JSON.stringify({ status }),
    }),
  remove: (id: number) => request<void>(`/applications/${id}`, { method: "DELETE" }),
};