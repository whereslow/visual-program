<script lang="ts">
import { invoke } from '@tauri-apps/api';

export default {
  name: 'App',
  components: {

  },
  data() {
    return {
      path: ""
    };
  },
  methods: {
    async buttonClick() {
      await invoke('open_about_window', { path: this.path })
    }
  }
}
</script>

<template>
  <router-view></router-view>
  <div class="footer">
    <h4>路径名称</h4>
    <input v-model="path" />
    <button @click="buttonClick">打开路径窗口</button>
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

.footer {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  background-color: #f5f5f5;
  padding: 10px;
  display: flex;
  justify-content: space-evenly;
  align-items: center;
}
</style>
