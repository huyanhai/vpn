<script setup lang="ts">
import { ref } from "vue";
import Git from "./components/Git.vue";
import { ElTabs, ElTabPane } from "element-plus";

const active = ref("git");
</script>

<template>
  <ElTabs v-model="active">
    <ElTabPane label="git" name="git">
      <Git :type="1" />
    </ElTabPane>
    <ElTabPane label="ssh" name="ssh">
      <Git :type="0" />
    </ElTabPane>
  </ElTabs>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
