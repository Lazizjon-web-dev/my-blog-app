<script setup lang="ts">
import VerticalCard from "@/components/VerticalCard.vue";
</script>

<template>
    <main class="">
        <div
            class="m-10 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 justify-items-center gap-2 md:mx-32 xl:grid-cols-4 xl:mx-16"
        >
            <VerticalCard v-for="i in 12" :key="i" :id="i"></VerticalCard>
        </div>
    </main>
</template>
