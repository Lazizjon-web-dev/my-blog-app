<template>
    <div
        class="max-w-sm bg-white border border-gray-200 rounded-lg shadow-sm dark:bg-gray-800 dark:border-gray-700"
    >
        <router-link :to="`/posts/${id}`">
            <img
                class="rounded-t-lg"
                src="https://flowbite.com/docs/images/blog/image-1.jpg"
                alt="post image"
            />
        </router-link>
        <div class="p-5">
            <router-link :to="`/posts/${id}`">
                <h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">
                    Noteworthy technology acquisitions {{ title }}
                </h5>
            </router-link>
            <p class="mb-3 font-normal text-gray-700 dark:text-gray-400">
                Here are the biggest enterprise technology acquisitions of 2021 so far, in reverse
                chronological order.
                {{ truncate(deltaToText(JSON.parse(content ?? "")), descriptionLength) }}
            </p>
            <router-link
                :to="`/posts/${id}`"
                class="inline-flex items-center px-3 py-2 text-sm font-medium text-center text-white bg-blue-700 rounded-lg hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
            >
                Read more
                <RightArrowIcon></RightArrowIcon>
            </router-link>
        </div>
    </div>
</template>

<script setup lang="ts">
import { truncate } from "@/utils/stringUtils";
import { deltaToText } from "@/utils/deltaRenderer";
import { usePostStore } from "@/stores/postStore";
import RightArrowIcon from "@/components/icons/RightArrowIcon.vue";

const descriptionLength = 105;
const store = usePostStore();
const { id, user_id, title, content } = defineProps({
    id: Number,
    user_id: Number,
    title: String,
    content: String,
});
</script>