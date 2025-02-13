import { defineStore } from 'pinia';
import api from '@/services/api';

export const useUserStore = defineStore('user', {
  state: () => ({
    profile: null,
  }),
  actions: {
    async fetchProfile(userId) {
      const response = await api.get(`/api/users/${userId}`);
      this.profile = response.data;
    },
    async updateProfile(profileData) {
      const response = await api.put('/api/users/me', profileData);
      this.profile = response.data;
    },
  },
});
