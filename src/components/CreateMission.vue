<template>
  <!-- Mission Progress -->
  <h2>Create Mission</h2>

  <div>Current Step: {{ currentStep }} of {{ totalSteps }}</div>

  <!-- Step 1 -->
  <div v-if="currentStep === 1">
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
  </div>

  <!-- Step 2 -->
  <div v-if="currentStep === 2">
    <label for="status">Status:</label>
    <select id="status" v-model="missionData.status" @change="updateMissionData($event)" required>
      <option value="Active">Active</option>
      <option value="Inactive">Inactive</option>
      <option value="Complete">Complete</option>
      <option value="Failed">Failed</option>
    </select>

    <button @click="submitMission">Submit Mission</button>
  </div>

  <!-- Submission -->
  <div v-if="isSubmitted">
    <h3>Mission Submitted Successfully!</h3>
    <p>Your mission has been received.</p>
    <button @click="reset">Reset Mission</button>
  </div>

  <!-- Buttons -->
  <div v-if="!isSubmitted">
    <button @click="previousStep">Previous</button>
    <button @click="nextStep">Next</button>
    <button @click="reset">Reset Mission</button>
  </div>
</template>

<script setup lang="ts">
import { getState, subscribe } from "../stores/MissionStore";
import { onMounted, ref } from "vue";
import { MissionDataStruct, MissionStatus } from "../lib/bindings";

// Initialize refs for variables used in template
const missionData = ref(getState().mission_data);
const isSubmitted = ref(getState().is_submitted);

// Create handler methods for mission actions
const nextStep = async () => await getState().nextStep();
const previousStep = async () => await getState().previousStep();
const reset = async () => await getState().reset();
const submitMission = async () => await getState().submitMission();

// Handler for updating mission data
const updateMissionData = async (e: Event) => {
  const target = e.target as HTMLInputElement | HTMLSelectElement;
  console.log("HELP ME PLEASE");
  console.log(target.id, target.value);

  // Update the local mission data object
  // missionData.value[target.id as keyof MissionDataStruct] = target.value as MissionStatus;
  
  if (target.id === "status") {
    missionData.value[target.id as keyof MissionDataStruct] = target.value as MissionStatus;
  } else {
    missionData.value[target.id as keyof Omit<MissionDataStruct, "status">] = target.value;
  }

  // Call update mission data method on Tauri, passing the updated mission data
  await getState().updateMissionData(getState().mission_data);
};

// When the component is mounted/loaded in DOM
onMounted(() => {
  // Subscribe to changes in the mission store to keep the UI in sync
  subscribe(() => {
    currentStep.value = getState().current_step;
    totalSteps.value = getState().total_steps;
    missionData.value = getState().mission_data;
    isSubmitted.value = getState().is_submitted;
  });
});
</script>
