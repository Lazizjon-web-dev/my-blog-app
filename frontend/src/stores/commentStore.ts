import { defineStore } from "pinia";
import { ref } from "vue";
import api from "@/services/api";

export type Comment = {
    id: number;
    user_id: number;
    post_id: number;
    content: string;
    created_at: Date;
};

export const useCommentStore = defineStore("comment", () => {
    //? State
    const comments = ref<Comment[][]>([]);
    //? Actions
    async function fetchCommentsByPostId(post_id: number) {
        const response = await api.get(`/api/posts/${post_id}/comments`);
        if (response.status === 200) {
            comments.value[post_id] = response.data;
        }
        console.log(comments.value);
        console.log(response);
    }
    async function postComment(post_id: number, token: string, content: string) {
        const response = await api.post(
            `/api/posts/${post_id}/comments`,
            { content },
            {
                headers: {
                    Authorization: token,
                },
            },
        );
        if (response.status === 200) {
            fetchCommentsByPostId(post_id);
        }
        console.log(response);
    }
    //? Return state and actions
    return { comments, fetchCommentsByPostId, postComment };
});
