import api from './api';
import type { AuthRequest, AuthResponse } from '../types';

export const authService = {
  async login(data: AuthRequest): Promise<AuthResponse> {
    const response = await api.post<AuthResponse>('/auth/authenticate', data);
    return response.data;
  },

  logout() {
    localStorage.removeItem('admin_token');
    localStorage.removeItem('admin_user_id');
  },

  isAuthenticated(): boolean {
    return !!localStorage.getItem('admin_token');
  },
};

