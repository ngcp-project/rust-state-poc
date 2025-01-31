<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

const greetMsg = ref("");
const name = ref("");

async function getWelcomeMessage() {
  console.log("getting welcome message");
  greetMsg.value = await invoke("get_welcome_message");
}

async function greet() {
  greetMsg.value = await invoke("greet", { name: name.value });
}

onMounted(async () => {
  await getWelcomeMessage();
  // Listen for welcome_message_changed events
  await listen("welcome_message_changed", (event: any) => {
    greetMsg.value = event.payload.welcome_message;
  });
});
</script>

<template>
  <div>
    <form class="row" @submit.prevent="greet">
      <input id="greet-input" v-model="name" placeholder="Enter a name..." />
      <button type="submit">Greet</button>
    </form>
    <p>{{ greetMsg }}</p>
  </div>
</template>
