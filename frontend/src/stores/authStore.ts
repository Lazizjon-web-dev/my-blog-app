import { defineStore } from "pinia";
import { ref } from "vue";
import api from "@/services/api";

type User = {
    id: number;
    email: string;
    username: string;
    created_at: Date;
};
export const useAuthStore = defineStore("auth", () => {
    //? State
    let id = localStorage.getItem("id");
    let email = localStorage.getItem("email");
    let username = localStorage.getItem("username");
    let created_at = localStorage.getItem("created_at");
    let tempUser: User | null = null;
    if (id && email && username && created_at) {
        tempUser = {
            id: +id,
            email,
            username,
            created_at: new Date(created_at),
        };
    }
    const user = ref<User | null>(tempUser || null);
    const token = ref<string | null>(localStorage.getItem("token") || null);

    //? Actions
    async function login(credentials: { email: string; password: string }) {
        const response = await api.post("/api/auth/login", credentials);
        console.log(response);
        if (response.status === 200) {
            token.value = response.data.token;
            user.value = response.data.user;
            for (let key in response.data.user) {
                let element = response.data.user[key];
                localStorage.setItem(key, element);
            }
            localStorage.setItem("token", response.data.token);
            if (!user) {
                const response = await api.post("/api/", token.value);
            }
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
        let items = ["token", "id", "email", "username", "created_at"];
        items.forEach((item) => localStorage.removeItem(item));
    }

    //? Return state and actions
    return { user, token, login, register, logout };
});
