import axios from "axios";

const API_URL = process.env.NEXT_PUBLIC_API_URL ?? "http://localhost:8080";

export const api = axios.create({
  baseURL: API_URL,
  headers: {
    "Content-Type": "application/json",
  },
});

// Request interceptor: attach Bearer token from authStore
api.interceptors.request.use((config) => {
  // Read token from zustand persisted storage (localStorage)
  // We access it via the persisted key to avoid circular imports
  if (typeof window !== "undefined") {
    try {
      const raw = localStorage.getItem("conquest-auth");
      if (raw) {
        const parsed = JSON.parse(raw) as { state?: { token?: string } };
        const token = parsed?.state?.token;
        if (token) {
          config.headers.Authorization = `Bearer ${token}`;
        }
      }
    } catch {
      // Ignore parse errors
    }
  }
  return config;
});

// Response interceptor: surface error messages
api.interceptors.response.use(
  (response) => response,
  (error) => {
    const message =
      error?.response?.data?.message ??
      error?.response?.data?.error ??
      error?.message ??
      "An unexpected error occurred.";
    return Promise.reject(new Error(message));
  }
);
