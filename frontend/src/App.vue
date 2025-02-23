<script setup lang="ts">
import { computed } from "vue";
import { useRoute, RouterView } from "vue-router";
import Navbar from "./components/Navbar.vue";

const route = useRoute();
const showNavbar = computed(() => !["login", "register"].includes(route.name?.toString() || ""));
</script>

<template>
    <Navbar v-if="showNavbar"></Navbar>
    <RouterView />
</template>