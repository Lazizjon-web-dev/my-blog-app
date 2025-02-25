<template>
    <nav class="bg-white border-gray-200 dark:bg-gray-900 sticky">
        <div class="max-w-screen-xl flex flex-wrap items-center justify-between mx-auto p-4">
            <router-link to="/" class="flex items-center space-x-3 rtl:space-x-reverse">
                <img src="../assets/logo.svg" class="h-8" alt="Blog Logo" />
                <span class="self-center text-2xl font-semibold whitespace-nowrap dark:text-white"
                    >My Blog</span
                >
            </router-link>
            <div class="flex items-center md:order-2 space-x-3 md:space-x-0 rtl:space-x-reverse">
                <UserDropdownMenu v-if="token"></UserDropdownMenu>
                <ul v-else class="flex">
                    <li>
                        <router-link
                            to="/login"
                            class="block py-2 px-3 text-gray-900 rounded-sm hover:bg-gray-100 md:hover:bg-transparent md:hover:text-blue-700 md:p-0 dark:text-white md:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent dark:border-gray-700"
                            >Login</router-link
                        >
                    </li>
                    <span class="text-gray-900 dark:text-white mx-2"> | </span>
                    <li>
                        <router-link
                            to="/register"
                            class="block py-2 px-3 text-gray-900 rounded-sm hover:bg-gray-100 md:hover:bg-transparent md:hover:text-blue-700 md:p-0 dark:text-white md:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent dark:border-gray-700"
                            >Register</router-link
                        >
                    </li>
                </ul>
                <button
                    data-collapse-toggle="navbar-user"
                    type="button"
                    class="inline-flex items-center p-2 w-10 h-10 justify-center text-sm text-gray-500 rounded-lg md:hidden hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600"
                    aria-controls="navbar-user"
                    aria-expanded="false"
                >
                    <span class="sr-only">Open main menu</span>
                    <HamburgerMenuIcon></HamburgerMenuIcon>
                </button>
            </div>
            <div
                class="items-center justify-between hidden w-full md:flex md:w-auto md:order-1"
                id="navbar-user"
            >
                <ul
                    class="flex flex-col font-medium p-4 md:p-0 mt-4 border border-gray-100 rounded-lg bg-gray-50 md:space-x-8 rtl:space-x-reverse md:flex-row md:mt-0 md:border-0 md:bg-white dark:bg-gray-800 md:dark:bg-gray-900 dark:border-gray-700"
                >
                    <li>
                        <router-link
                            to="/"
                            class="block py-2 px-3 text-white bg-blue-700 rounded-sm md:bg-transparent md:text-blue-700 md:p-0 md:dark:text-blue-500"
                            >Home</router-link
                        >
                    </li>
                    <li>
                        <router-link
                            to="/create"
                            class="block py-2 px-3 text-gray-900 rounded-sm hover:bg-gray-100 md:hover:bg-transparent md:hover:text-blue-700 md:p-0 dark:text-white md:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent dark:border-gray-700"
                            >Create</router-link
                        >
                    </li>
                    <li>
                        <router-link
                            to="/"
                            class="block py-2 px-3 text-gray-900 rounded-sm hover:bg-gray-100 md:hover:bg-transparent md:hover:text-blue-700 md:p-0 dark:text-white md:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent dark:border-gray-700"
                            >About</router-link
                        >
                    </li>
                </ul>
            </div>
        </div>
    </nav>

    <!-- Logged-Out User:
Copy

+-----------------------------------------------------------------------------+
| MyBlog | Home | Create | About |                 Login/Sign Up              |
+-----------------------------------------------------------------------------+

Logged-In User:
Copy

+-----------------------------------------------------------------------------+
| MyBlog | Home | Categories | About | Create Post | Profile ▼                |
+-----------------------------------------------------------------------------+

Dropdown Menu for Profile:
Copy

+-----------------------------------------------------------------------------+
| MyBlog | Home | Categories | About | Create Post | Profile ▼                |
+-----------------------------------------------------------------------------+
                                                  |
                                                  v
                                              +-------------------+
                                              | My Profile        |
                                              | Settings          |
                                              | Logout            |
                                              +-------------------+ -->
</template>

<script setup lang="ts">
import { ref, computed } from "vue";
import { useAuthStore } from "../stores/authStore";
import HamburgerMenuIcon from "./icons/HamburgerMenuIcon.vue";
import UserDropdownMenu from "./UserDropdownMenu.vue";
const store = useAuthStore();
const token = computed(() => store.token);
</script>