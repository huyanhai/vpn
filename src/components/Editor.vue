<template>
  <div ref="editorContainer" class="editor"></div>
</template>
<script lang="ts" setup>
import { editor } from "monaco-editor";
import { onMounted, onUnmounted, ref, watch } from "vue";
const editorContainer = ref<HTMLElement>();

let editorInstance: editor.IStandaloneCodeEditor;

const props = defineProps<{
  options: editor.IStandaloneEditorConstructionOptions;
}>();

const emits = defineEmits(["update:options"]);

const initEditor = () => {
  editorInstance = editor.create(editorContainer.value as HTMLElement, {
    language: "javascript",
    automaticLayout: true,
    scrollbar: {
      alwaysConsumeMouseWheel: false,
    },
    ...props.options,
  });

  editorInstance.onDidChangeModelContent(() => {
    emits("update:options", {
      ...props,
      value: editorInstance.getValue(),
    });
  });

  editorInstance.onDidBlurEditorWidget(() => {});
};

watch(
  () => props.options.value,
  (val) => {
    if (val !== editorInstance.getValue()) {
      editorInstance.setValue(val as string);
    }
  }
);

onMounted(() => {
  initEditor();
});

onUnmounted(() => {
  if (editorInstance) {
    editorInstance.dispose();
  }
});
</script>

<style lang="scss" scoped>
.editor {
  width: 100%;
  min-height: 300px;
  height: 100%;
}
</style>
