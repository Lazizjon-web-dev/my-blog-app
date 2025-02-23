<template>
    <div class="flex w-auto mx-48 mt-4">
        <div class="flex-3 bg-amber-300 rounded-2xl mr-4">
            <div class="flex rounded">
                <div class="p-2">
                    <img
                        class="w-16 h-16 rounded-full"
                        src="https://flowbite.com/docs/images/people/profile-picture-1.jpg"
                        alt="user photo"
                    />
                </div>
                <div class="ml-4 flex-auto">
                    <div class="text-2xl font-semibold">User name</div>
                </div>
            </div>
            <h5 class="bg-purple-400 text-3xl font-semibold font-sans uppercase pl-8">
                {{ post?.title }}
            </h5>
            <div class="bg-blue-400">{{ post?.content }}</div>
            <div class="bg-purple-400">
                <div class="p-2 flex">
                    <img
                        class="w-10 h-10 rounded-full"
                        src="https://flowbite.com/docs/images/people/profile-picture-1.jpg"
                        alt="user photo"
                    />
                    <input
                        v-model="comment"
                        @keypress.enter="postComment"
                        type="text"
                        class="flex-auto"
                    />
                    <button @click="postComment">Comment</button>
                </div>
            </div>
            <div class="bg-green-400">
                <p v-for="comment in comments" :key="comment.id">
                    User with id {{ comment.user_id }} says:<br />
                    {{ comment.content }}
                </p>
            </div>
        </div>
        <div class="flex-1 bg-green-500">this is side</div>
    </div>
    <div>This is post view</div>
</template>

<script setup lang="ts">
//? Import statements
import { ref, computed, onBeforeMount, type ComputedRef } from "vue";
import { useRoute } from "vue-router";
import { usePostStore, type Post } from "@/stores/postStore";
import { useAuthStore } from "@/stores/authStore";
import { useCommentStore } from "@/stores/commentStore";

//? Installing stores
const postStore = usePostStore();
const authStore = useAuthStore();
const commentStore = useCommentStore();

//? Extracting id of the post from
const route = useRoute();
const id = computed(() => route.params.id);

//? Creating variables for store data locally
const post = ref<Post | null>(null);
const comments = computed(() => commentStore.comments.find((_, ind) => ind == +id.value));
const comment = ref("");

//? Defining Lifecycle hooks
onBeforeMount(() => {
    postStore.currentPost =
        postStore.posts.find((post) => post.id.toString() === id.value) ?? fetchPostById(id);
    post.value = postStore.currentPost;
    commentStore.fetchCommentsByPostId(+id.value);
});

//? Defining Helper functions
function postComment() {
    if (authStore.token && comment.value && id.value) {
        commentStore.postComment(+id.value, authStore.token, comment.value);
        comment.value = "";
    }
}
function fetchPostById(id: ComputedRef<string | string[]>): Post | null {
    postStore.fetchPostById(+id.value);
    return postStore.currentPost;
}
</script>
