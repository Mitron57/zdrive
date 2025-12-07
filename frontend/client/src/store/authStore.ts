import { create } from 'zustand';

interface AuthState {
  token: string | null;
  userId: string | null;
  setAuth: (token: string, userId: string) => void;
  clearAuth: () => void;
}

export const useAuthStore = create<AuthState>((set) => ({
  token: localStorage.getItem('token'),
  userId: localStorage.getItem('user_id'),
  setAuth: (token: string, userId: string) => {
    localStorage.setItem('token', token);
    localStorage.setItem('user_id', userId);
    set({ token, userId });
  },
  clearAuth: () => {
    localStorage.removeItem('token');
    localStorage.removeItem('user_id');
    set({ token: null, userId: null });
  },
}));

