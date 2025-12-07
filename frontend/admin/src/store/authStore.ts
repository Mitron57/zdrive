import { create } from 'zustand';

interface AuthState {
  token: string | null;
  userId: string | null;
  setAuth: (token: string, userId: string) => void;
  clearAuth: () => void;
}

export const useAuthStore = create<AuthState>((set) => ({
  token: localStorage.getItem('admin_token'),
  userId: localStorage.getItem('admin_user_id'),
  setAuth: (token: string, userId: string) => {
    localStorage.setItem('admin_token', token);
    localStorage.setItem('admin_user_id', userId);
    set({ token, userId });
  },
  clearAuth: () => {
    localStorage.removeItem('admin_token');
    localStorage.removeItem('admin_user_id');
    set({ token: null, userId: null });
  },
}));

