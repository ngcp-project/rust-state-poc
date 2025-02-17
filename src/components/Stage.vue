<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
async function transitionNextStage() {
  console.log("Transition button!");
  await invoke("transition_next_stage");
}

</script>

<template>
  <div>
    <h1>Stage</h1>

    <button @click="transitionNextStage">Transition Next Stage</button>
  </div>
</template>
