import { defineStore } from "pinia";
import { ref } from "vue";
import api from "@/services/api";

export const useAuthStore = defineStore("auth", () => {
    //? State
    const user = ref<Object | null>(null);
    const token = ref<string | null>(localStorage.getItem("token") || null);

    //? Actions
    async function login(credentials: { email: string; password: string }) {
        const response = await api.post("/api/auth/login", credentials);
        console.log(response);
        if (response.status === 200) {
            token.value = response.data.token;
            localStorage.setItem("token", response.data.token);
        }
    }
    async function register(credentials: { username: string; email: string; password: string }) {
        const response = await api.post("/api/auth/register", credentials);
        console.log(response);
        if (response.status === 200) {
            user.value = response.data.data;
            login({ email: credentials.email, password: credentials.password });
        }
    }
    function logout() {
        user.value = null;
        token.value = null;
        localStorage.removeItem("token");
    }

    //? Return state and actions
    return { user, token, login, register, logout };
});
