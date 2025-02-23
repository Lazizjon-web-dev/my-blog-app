<template>
    <div class="flex justify-center items-center h-screen w-screen">
        <form
            @submit.prevent="register"
            class="max-w-sm mx-auto p-4 bg-white border border-gray-200 rounded-lg shadow-sm space-y-6 sm:p-6 md:p-8 dark:bg-gray-800 dark:border-gray-700"
        >
            <h5 class="text-xl font-medium text-gray-900 dark:text-white">Create Account</h5>
            <div>{{ errorMsg }}</div>
            <!-- <div
                v-for="{ label, type, placeholder, id, isRequired } in inputs"
                :key="id"
                class="mb-5"
            >
                <label
                    :for="id"
                    class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                    >{{ label }}</label
                >
                <input
                    :type="type"
                    :id="id"
                    v-model="registerRequest[id]"
                    class="shadow-xs bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500 dark:shadow-xs-light"
                    :placeholder="placeholder"
                    :required="isRequired"
                />
            </div> -->
            <div class="mb-5">
                <label
                    for="username"
                    class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                    >Your username</label
                >
                <input
                    v-model="registerRequest.username"
                    type="text"
                    id="username"
                    class="shadow-xs bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500 dark:shadow-xs-light"
                    placeholder=""
                    required
                />
            </div>
            <div class="mb-5">
                <label
                    for="email"
                    class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                    >Your email</label
                >
                <input
                    v-model="registerRequest.email"
                    type="email"
                    id="email"
                    class="shadow-xs bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500 dark:shadow-xs-light"
                    placeholder="name@flowbite.com"
                    required
                />
            </div>
            <div class="mb-5">
                <label
                    for="password"
                    class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                    >Your password</label
                >
                <input
                    v-model="registerRequest.password"
                    type="password"
                    id="password"
                    class="shadow-xs bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500 dark:shadow-xs-light"
                    required
                />
            </div>
            <div class="mb-5">
                <label
                    for="repeat-password"
                    class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                    >Repeat password</label
                >
                <input
                    v-model="registerRequest.repeat_password"
                    type="password"
                    id="repeat-password"
                    class="shadow-xs bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500 dark:shadow-xs-light"
                    required
                />
            </div>
            <div class="flex items-start mb-5">
                <div class="flex items-center h-5">
                    <input
                        id="terms"
                        type="checkbox"
                        value=""
                        class="w-4 h-4 border border-gray-300 rounded-sm bg-gray-50 focus:ring-3 focus:ring-blue-300 dark:bg-gray-700 dark:border-gray-600 dark:focus:ring-blue-600 dark:ring-offset-gray-800 dark:focus:ring-offset-gray-800"
                        required
                    />
                </div>
                <label for="terms" class="ms-2 text-sm font-medium text-gray-900 dark:text-gray-300"
                    >I agree with the
                    <a href="#" class="text-blue-600 hover:underline dark:text-blue-500"
                        >terms and conditions</a
                    ></label
                >
            </div>
            <button
                type="submit"
                class="w-full text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
            >
                Register new account
            </button>
            <div class="text-sm font-medium text-gray-500 dark:text-gray-300">
                Already have an account?
                <router-link to="/login" class="text-blue-700 hover:underline dark:text-blue-500">
                    Log in
                </router-link>
            </div>
        </form>
    </div>
</template>

<script setup lang="ts">
import { ref, reactive } from "vue";
import { useAuthStore } from "@/stores/authStore";
import router from "@/router";

const store = useAuthStore();
const errorMsg = ref("");
const registerRequest = reactive({
    username: "test",
    email: "test@mail.com",
    password: "123",
    repeat_password: "123",
});
function register() {
    if (
        registerRequest.username &&
        registerRequest.email &&
        registerRequest.password &&
        registerRequest.password === registerRequest.repeat_password
    ) {
        let respose = store.register({
            username: registerRequest.username,
            email: registerRequest.email,
            password: registerRequest.password,
        });
        respose.then(
            () => router.replace("/"),
            () => (errorMsg.value = "User already exists")
        );
    }
}
</script>