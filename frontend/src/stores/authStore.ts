import { create } from "zustand";
import { persist } from "zustand/middleware";
import axios from "axios";

const API_URL = process.env.NEXT_PUBLIC_API_URL ?? "http://localhost:8080";

interface User {
  id: string;
  username: string;
  email: string;
  universe_id?: string;
}

interface AuthState {
  user: User | null;
  token: string | null;
  login: (email: string, password: string) => Promise<void>;
  register: (username: string, email: string, password: string) => Promise<void>;
  logout: () => void;
  setToken: (token: string) => void;
}

export const useAuthStore = create<AuthState>()(
  persist(
    (set) => ({
      user: null,
      token: null,

      login: async (email, password) => {
        const res = await axios.post(`${API_URL}/api/auth/login`, {
          email,
          password,
        });
        const { token, user } = res.data;
        set({ token, user });
      },

      register: async (username, email, password) => {
        const res = await axios.post(`${API_URL}/api/auth/register`, {
          username,
          email,
          password,
        });
        const { token, user } = res.data;
        set({ token, user });
      },

      logout: () => {
        set({ user: null, token: null });
      },

      setToken: (token) => {
        set({ token });
      },
    }),
    {
      name: "conquest-auth",
      partialize: (state) => ({ token: state.token, user: state.user }),
    }
  )
);
