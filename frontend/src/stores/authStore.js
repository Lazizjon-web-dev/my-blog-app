import { defineStore } from 'pinia';
import api from '@/services/api';

export const useAuthStore = defineStore('auth', {
  state: () => ({
    user: null,
    token: localStorage.getItem('token') || null,
  }),
  actions: {
    async login(credentials) {
      const response = await api.post('/api/auth/login', credentials);
      this.user = response.data.user;
      this.token = response.data.token;
      localStorage.setItem('token', this.token);
    },
    async register(userData) {
      const response = await api.post('/api/auth/register', userData);
      this.user = response.data.user;
      this.token = response.data.token;
      localStorage.setItem('token', this.token);
    },
    logout() {
      this.user = null;
      this.token = null;
      localStorage.removeItem('token');
    },
  },
});
