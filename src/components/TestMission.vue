<template>
    <div>
        <h2>Current Mission Stage: {{ currentMissionStage }}</h2>
        <button @click="transitionNextStage()">Transition Next Stage</button>
    </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue';
import { getState, subscribe } from "../stores/TestMissionStore.ts";


// Initialize count ref
const currentMissionStage = ref(getState().stages[getState().current_stage_id]);

// Update count using zustand increaseCount
// which is linked to taurRPC increaseCount binding
// which is linked to tauri increase_count command
const transitionNextStage = () => {
    getState().transitionNextStage();
};

// When component is mounted/loaded in DOM
onMounted(() => {
    // reinitialize count to be what is in current store
    // in order to keep track when tauRPC updates count
    currentMissionStage.value = getState().stages[getState().current_stage_id];
    
    // Initialize store subscribe to update count on store update
    subscribe(() => {
        currentMissionStage.value = getState().stages[getState().current_stage_id];
    });
    
});

// Unsubscribe from store once component is unmounted
onUnmounted(() => {
    subscribe();
});


</script>