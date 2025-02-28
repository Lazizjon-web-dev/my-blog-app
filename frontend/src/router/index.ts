import { createRouter, createWebHistory } from "vue-router";
import HomeView from "@/views/HomeView.vue";
import LoginView from "@/views/LoginView.vue";
import RegisterView from "@/views/RegisterView.vue";
import PostView from "@/views/PostView.vue";
import CreateView from "@/views/CreateView.vue";

const routes = [
    { path: "/", name: "home", component: HomeView },
    { path: "/login", name: "login", component: LoginView },
    { path: "/register", name: "register", component: RegisterView },
    { path: "/posts/:id", name: "post", component: PostView },
    { path: "/create", name: "create", component: CreateView },
    // {
    //     path: "/about",
    //     name: "about",
    //?     // route level code-splitting
    //?     // this generates a separate chunk (About.[hash].js) for this route
    //?     // which is lazy-loaded when the route is visited.
    //     component: () => import("../views/AboutView.vue"),
    // },
];

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes,
});

export default router;
