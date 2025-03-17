<template>
  <!-- Mission Progress -->
  <h2>Create Mission</h2>

  <label for="missionName">Mission Name:</label>
  <input
    type="text"
    id="missionName"
    v-model="missionData.mission_name"
    @change="updateMissionData($event)"
    required
  />

  <div>
    <label for="keepInZone">Keep In Zone:</label>
    <input
      type="text"
      id="keepInZone"
      v-model="missionData.keep_in_zone"
      @change="updateMissionData($event)"
      required
    />

    <label for="keepOutZone">Keep Out Zone:</label>
    <input
      type="text"
      id="keepOutZone"
      v-model="missionData.keep_out_zone"
      @change="updateMissionData($event)"
      required
    />
  </div>

  <label for="status">Status:</label>
  <select id="status" v-model="missionData.status" @change="updateMissionData($event)" required>
    <option value="Active">Active</option>
    <option value="Inactive">Inactive</option>
    <option value="Complete">Complete</option>
    <option value="Failed">Failed</option>
  </select>

  <button @click="submitMission">Submit Mission</button>

  <!-- Submission -->
  <div v-if="isSubmitted">
    <h3>Mission Submitted Successfully!</h3>
    <p>Your mission has been received.</p>
    <button @click="reset">Reset Mission</button>
  </div>
</template>

<script setup lang="ts">
import { getState, subscribe } from "../stores/MissionStore";
import { onMounted, ref } from "vue";
import { MissionStruct, MissionStatus } from "../lib/bindings";

// Initialize refs for variables used in template
const missionData = ref(getState().mission_form_state);
const isSubmitted = ref(getState().is_submitted);

// Create handler methods for mission actions
const reset = async () => await getState().reset();
const submitMission = async () => await getState().submitMission();

// Handler for updating mission data
const updateMissionData = async (e: Event) => {
  const target = e.target as HTMLInputElement | HTMLSelectElement;

  // Update the local mission data object
  // missionData.value[target.id as keyof MissionStruct] = target.value as MissionStatus;

  if (target.id === "status") {
    missionData.value[target.id as keyof MissionStruct] = target.value as MissionStatus;
  } else {
    missionData.value[target.id as keyof Omit<MissionStruct, "status">] = target.value;
  }

  // Call update mission data method on Tauri, passing the updated mission data
  await getState().updateMissionData(getState().mission_form_state);
};

// When the component is mounted/loaded in DOM
onMounted(() => {
  // Subscribe to changes in the mission store to keep the UI in sync
  subscribe(() => {
    missionData.value = getState().mission_form_state;
    isSubmitted.value = getState().is_submitted;
  });
});
</script>
