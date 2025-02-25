<template>
    <div class="max-w-3xl my-0 mx-auto p-5">
        <header>
            <button
                class="py-2.5 px-5 border-none rounded bg-red-500 text-white cursor-pointer"
                @click="back"
            >
                &lt;- Back
            </button>
            <button
                class="py-2.5 px-5 border-none rounded bg-blue-500 text-white cursor-pointer"
                @click="save"
            >
                Save
            </button>
            <button
                class="py-2.5 px-5 border-none rounded bg-green-500 text-white cursor-pointer"
                @click="publish"
            >
                Publish
            </button>
        </header>

        <div>Create new post</div>
        <form class="flex flex-col">
            {{ post.content }}
            <input
                v-model="post.title"
                class="w-full text-4xl p-2.5 mb-5 outline-gray-200 rounded font-serif font-medium focus:outline"
                type="text"
                placeholder="Title"
            />
            <QuillEditor
                class="text-xl"
                theme="snow"
                v-model:content="post.content"
                contentType="delta"
                :options="editorOptions"
            >
                <!-- <template #toolbar>
                    <div id="my-toolbar">
                        <button class="ql-bold"></button><button class="ql"></button>
                    </div>
                </template> -->
            </QuillEditor>
        </form>
    </div>
</template>

<script setup lang="ts">
import { reactive } from "vue";
import { useAuthStore } from "@/stores/authStore";
import { usePostStore } from "@/stores/postStore";
import { QuillEditor } from "@vueup/vue-quill";

const authStore = useAuthStore();
const postStore = usePostStore();

const post = reactive({
    title: "",
    content: "",
});

const editorOptions = {
    theme: "snow",
    modules: {
        toolbar:
            // "#my-toolbar",
            [
                ["bold", "italic", "underline", "strike"], // Basic formatting
                [{ header: [1, 2, 3, false] }], // Headers
                ["blockquote", "code-block"], // Blockquote and code block
                [{ list: "ordered" }, { list: "bullet" }], // Lists
                ["link", "image"], // Links and images
                ["clean"], // Remove formatting
            ],
    },
};

function publish() {
    const token = authStore.token;
    if (token && post.title && post.content) {
        postStore.createPost({ title: post.title, content: JSON.stringify(post.content) }, token);
    }
}
function save() {}
function back() {}
</script>