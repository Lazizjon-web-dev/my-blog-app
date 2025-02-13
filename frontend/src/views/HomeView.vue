<template>
  <div>
    <h1>Blog Posts</h1>
    <ul>
      <li v-for="post in posts" :key="post.id">{{ post.title }}</li>
    </ul>
  </div>
</template>

<script>
import api from '@/services/api';

export default {
  data() {
    return {
      posts: [],
    };
  },
  async created() {
    const response = await api.get('/api/posts');
    this.posts = response.data;
  },
};
</script>
