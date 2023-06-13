<template>
  <ElSpace direction="vertical" :fill="true" style="width: 100%">
    <ElCard>
      <Editor :options="options" @update:options="updateOptions" />
    </ElCard>
    <ElButton style="width: inherit" type="primary" @click="saveFile">保存</ElButton>
    <ElButton style="width: inherit" type="primary" @click="run">运行</ElButton>
    <ElButton style="width: inherit" type="primary" @click="stop">停止</ElButton>
  </ElSpace>
</template>

<script setup lang="ts">
import { onMounted, ref, toRaw } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

import { ElButton, ElSpace, ElCard, ElMessage } from "element-plus";

import Editor from "./Editor.vue";

const gitCtx = ref("");
const options = ref({
  value: gitCtx,
});

const { type } = defineProps<{ type: 0 | 1 }>();

const updateOptions = (v: any) => {
  options.value = {
    ...toRaw(options),
    ...v,
  };
};

// async function greet() {
//   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//   greetMsg.value = await invoke("greet", { name: name.value });
// }

const gitFile = async () => {
  gitCtx.value = await invoke("git_config_ctx", { fileType: type });
};

const saveFile = async () => {
  await invoke("write_config", { ctx: options.value.value, fileType: type });
};

const run = async () => {
  const result = await invoke("running");
  if (result) {
    return ElMessage.success("启动成功");
  }
  return ElMessage.error("程序运行中");
};

const stop = async () => {
  await invoke("kill");
};

onMounted(() => {
  gitFile();
});
</script>
