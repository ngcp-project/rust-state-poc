<template>
  <div class="container">
    Count: {{ count }}

    <button @click="increaseCount">Increase count</button>
  </div>
</template>

<script lang="ts" setup>
import { ref } from "vue";
import { getState, subscribe } from "./lib/zustand/count";

const count = ref(getState().count);

const increaseCount = () => {
  getState().increaseCount();
};

subscribe(() => {
  count.value = getState().count;
});
</script>

<style scoped></style>
