<template>
    <div>
      <h2>Local Count: {{ count }}</h2>
      <button @click="increaseCount">Increase Local Count</button>
    </div>
  </template>
  
<script setup>

import { onMounted, onUnmounted, ref } from "vue";
import {getState, subscribe} from "../stores/LocalCountStore";

// Initialize count ref
const count = ref(getState().count);

// Update count using zustand increaseCount
const increaseCount = () => {
  getState().increaseCount();
};


// When component is mounted/loaded in DOM
onMounted(() => {
    // reinitialize count to be what is in current store
    count.value = getState().count;
    
    // Initialize store subscribe to update count on store update
    subscribe(() => {
        count.value = getState().count;
    });
})

// Unsubscribe from store once component is unmounted
onUnmounted(() => {
    subscribe();
});

</script>