// src/router/index.ts
import { createRouter, createWebHistory } from 'vue-router';

const routes = [
  { path: '/generate', name: 'Generate', component: () => import('../views/Generate.vue') },
  { path: '/', name: 'Design', component: () => import('../views/Design.vue') },
  { path: '/setting', name: 'Setting', component: () => import('../views/Setting.vue') },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
