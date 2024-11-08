<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { onMounted, ref } from "vue";

const count = ref(0);
async function getCount() {
  count.value = await invoke("get_count");
}

async function setCount(value: number) {
  console.log("setCount called");
  await invoke("set_count", { count: value });
}

function increaseCount() {
  setCount(count.value + 1);
}

onMounted(async () => {
  await getCount();
  // Listen for count_changed events
  await listen("count_changed", (event: any) => {
    count.value = event.payload.count;
  });
});
</script>

<template>
  <div>
    <div>Count: {{ count }}</div>
    <button @click="increaseCount">Increase Count</button>
  </div>
</template>
