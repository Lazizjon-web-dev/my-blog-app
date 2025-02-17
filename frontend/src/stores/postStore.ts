import { defineStore } from "pinia";
import { reactive, ref } from "vue";
import api from "@/services/api";

type Post = {
    id: number;
    user_id: number;
    title: string;
    content: string;
    created_at: Date;
    updated_at: Date;
};

export const usePostStore = defineStore("post", () => {
    //? State
    let posts = ref<Post[]>([]);
    let currentPost = ref<Post | null>(null);

    //? Actions
    async function fetchPosts() {
        const response = await api.get("/api/posts");
        posts.value = response.data;
    }
    async function fetchPostById(id: number) {
        const response = await api.get(`/api/posts/${id}`);
        currentPost.value = response.data;
    }

    //? Return state and actions
    return { posts, currentPost, fetchPosts, fetchPostById };
});
