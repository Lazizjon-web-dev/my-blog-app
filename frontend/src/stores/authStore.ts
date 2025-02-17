import { defineStore } from "pinia";
import { ref } from "vue";
import api from "@/services/api";

export const useAuthStore = defineStore("auth", () => {
    //? State
    const user = ref<Object | null>(null);
    const token = ref<string | null>(localStorage.getItem("token") || null);

    //? Actions
    async function login(email: string, password: string) {
        const response = await api.post("/api/auth/login", { email, password });
        user.value = response.data.user;
        token.value = response.data.token;
        localStorage.setItem("token", response.data.token);
    }
    async function register(username: string, email: string, password: string) {
        const response = await api.post("/api/auth/register", { username, email, password });
        user.value = response.data.user;
        token.value = response.data.token;
        localStorage.setItem("token", response.data.token);
    }
    function logout() {
        user.value = null;
        token.value = null;
        localStorage.removeItem("token");
    }

    //? Return state and actions
    return { user, token, login, register, logout };
});
