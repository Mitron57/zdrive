import api from './api';
import type { AuthRequest, AuthResponse, RegisterRequest } from '../types';

export const authService = {
  async register(data: RegisterRequest): Promise<AuthResponse> {
    const response = await api.post<AuthResponse>('/auth/register', data);
    return response.data;
  },

  async login(data: AuthRequest): Promise<AuthResponse> {
    const response = await api.post<AuthResponse>('/auth/authenticate', data);
    return response.data;
  },

  logout() {
    localStorage.removeItem('token');
    localStorage.removeItem('user_id');
  },

  isAuthenticated(): boolean {
    return !!localStorage.getItem('token');
  },
};

