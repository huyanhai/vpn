<template>
  <ElSpace direction="vertical" :fill="true" style="width: 100%">
    <div v-if="[1, 2].includes(type)">
      <ElPopover title="配置示例" :width="400">
        <textarea disabled :value="configOptions"></textarea>
        <template #reference>
          <ElLink @click="copy" type="primary" :underline="false">复制配置</ElLink>
        </template>
      </ElPopover>
      <div class="code" v-if="type === 2">
        # 首先安装转换工具<br />
        <ElLink type="success" :underline="false">npm install -g http-proxy-to-socks</ElLink><br />
        # 然后使用这个工具监听8002端口,支持http代理，然后所有8002的http代理数据都将转换成socks的代理数据发送到1081上<br />
        <ElLink type="success" :underline="false">hpts -s 127.0.0.1:1081 -p 8002</ElLink><br />
      </div>
    </div>
    <ElCard>
      <Editor :options="options" @update:options="updateOptions" />
    </ElCard>
    <ElButton style="width: inherit" type="primary" @click="saveFile">保存</ElButton>
    <ElButton style="width: inherit" type="success" @click="run">运行</ElButton>
    <ElButton style="width: inherit" type="danger" @click="stop">停止</ElButton>
  </ElSpace>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, toRaw } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

import { ElButton, ElSpace, ElCard, ElMessage, ElLink, ElPopover } from "element-plus";

import Editor from "./Editor.vue";

const gitCtx = ref("");
const options = ref({
  value: gitCtx,
});

const { type } = defineProps<{ type: 0 | 1 | 2 }>();

const configOptions = computed(() => {
  if (type === 1) {
    return `[http "https://git.cisdigital.cn/"]
	proxy = socks5://127.0.0.1:65522 
[https "https://git.cisdigital.cn/"] 
	proxy = socks5://127.0.0.1:65522`;
  }
  return `proxy=http://127.0.0.1:8002/ 
https-proxy=http://127.0.0.1:8002`;
});

const copy = () => {
  navigator.clipboard.writeText(configOptions.value);
  ElMessage.success("复制成功");
};

const updateOptions = (v: any) => {
  options.value = {
    ...toRaw(options),
    ...v,
  };
};

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

<style scoped lang="scss">
textarea,
.code {
  width: 100%;
  min-height: 100px;
  border: none;
  font-size: 14px;
  color: var(--el-text-color);
}
.code {
  display: block;
  color: rgba($color: #000000, $alpha: 0.5);
  font-size: 12px;

  p {
    margin: 0;
    padding: 0;
    display: block;
    color: #000000;
  }
}
</style>
