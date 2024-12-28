<template>
    <div>
        <h2>Shared Count: {{ count }}</h2>
        <button @click="increaseCount">Increase Shared Count</button>
    </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue';
import { getState, subscribe } from "../stores/SharedCountStore.ts";


// Initialize count ref
const count = ref(getState().count);

// Update count using zustand increaseCount
// which is linked to taurRPC increaseCount binding
// which is linked to tauri increase_count command
const increaseCount = () => {
    getState().increaseCount();
};

// When component is mounted/loaded in DOM
onMounted(() => {
    // reinitialize count to be what is in current store
    // in order to keep track when tauRPC updates count
    count.value = getState().count;
    
    // Initialize store subscribe to update count on store update
    subscribe(() => {
        count.value = getState().count;
    });
    
});

// Unsubscribe from store once component is unmounted
onUnmounted(() => {
    subscribe();
});


</script>