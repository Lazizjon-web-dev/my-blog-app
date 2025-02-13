import { defineStore } from 'pinia';
import api from '@/services/api';

export const usePostStore = defineStore('post', {
  state: () => ({
    posts: [],
    currentPost: null,
  }),
  actions: {
    async fetchPosts() {
      const response = await api.get('/api/posts');
      this.posts = response.data;
    },
    async fetchPostById(id) {
      const response = await api.get(`/api/posts/${id}`);
      this.currentPost = response.data;
    },
    async createPost(postData) {
      const response = await api.post('/api/posts', postData);
      this.posts.push(response.data);
    },
    async updatePost(id, postData) {
      const response = await api.put(`/api/posts/${id}`, postData);
      const index = this.posts.findIndex((post) => post.id === id);
      if (index !== -1) {
        this.posts[index] = response.data;
      }
    },
    async deletePost(id) {
      await api.delete(`/api/posts/${id}`);
      this.posts = this.posts.filter((post) => post.id !== id);
    },
  },
});
