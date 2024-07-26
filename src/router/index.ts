// src/router/index.ts
import { createRouter, createWebHistory } from 'vue-router';
import Design from '../views/Design.vue';
import Setting from '../views/Setting.vue';
import Generate from '../views/Generate.vue';

const routes = [
  { path: '/generate', name: 'Generate', component: Generate },
  { path: '/', name: 'Design', component: Design },
  { path: '/setting', name: 'Setting', component: Setting },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
