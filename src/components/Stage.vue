<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
async function transitionNextStage() {
  console.log("I clicked button!");
  await invoke("test_cmd");
}

</script>

<template>
  <div>
    <h1>Stage</h1>

    <button @click="transitionNextStage">Transition Next Stage</button>
  </div>
</template>
