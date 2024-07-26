import { createApp } from "vue";
import "./styles.css";
import vue3lottie from "vue3-lottie";
import App from "./App.vue";
import router from './router/index';
createApp(App).use(vue3lottie).use(router).mount("#app");
