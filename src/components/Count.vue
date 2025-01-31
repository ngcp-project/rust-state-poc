<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { onMounted, ref } from "vue";

// Reactive variable for the count
// const count = ref(0);

// Reactive variable for telemetry data
const defaultTelemetry = {
  localIP: "",
  pitch: 0,
  yaw: 0,
  roll: 0,
  speed: 0,
  altitude: 0,
  batteryLife: 0,
  currentPosition: { latitude: 0, longitude: 0 },
  lastUpdated: "",
  fireFound: false,
  fireCoordinate: { latitude: 0, longitude: 0 }
};
const telemetry = ref({ ...defaultTelemetry });

// var for most recent stage
const defaultMostRecentStage = {
  stageId: 0,
  stageName: "",
  vehicleName: "",
};
const mostRecentStage = ref({ ...defaultMostRecentStage });

// Function to get the count from the backend
// async function getCount() {
//   count.value = await invoke("get_count");
// }

// // Function to set the count in the backend
// async function setCount(value: number) {
//   console.log("setCount called");
//   await invoke("set_count", { count: value });
// }

// // Function to increase the count
// function increaseCount() {
//   setCount(count.value + 1);
// }

// Function to get the telemetry data from the backend
async function getTelemetry() {
  telemetry.value = Object.assign({}, defaultTelemetry, await invoke("get_telemetry"));
  console.log("I am hungry", telemetry.value);
}

// Function to update the telemetry data on the backend
async function setTelemetry(newTelemetry: any) {
  console.log("setTelemetry called", newTelemetry);
  await invoke("set_telemetry", { telemetry: newTelemetry });
}

// get most recent stage
async function getMostRecentStage() {
  mostRecentStage.value = await Object.assign(await invoke("set_most_recent_stage"));
  console.log("most recent stage value:", mostRecentStage.value);
}

// Listen for telemetry (and other) updates from the backend
onMounted(async () => {
  // await getCount(); // Get initial count
  await getTelemetry(); // Get initial telemetry data
  
  // Listen for count_changed events and update count
  // await listen("count_changed", (event: any) => {
  //   count.value = event.payload.count;
  // });

  await listen("most_recent_stage_update", (event: any) => {
    console.log("Received stage update", event.payload);
    mostRecentStage.value.stageId = event.payload.stageId;
    mostRecentStage.value.stageName = event.payload.stageName;
    mostRecentStage.value.vehicleName = event.payload.vehicleName;
    
  });

  // Listen for telemetry_changed events and update telemetry data
  await listen("telemetry_update", (event: any) => {
    console.log("Received telemetry update", event.payload);
    // telemetry.value = event.payload.telemetry;
    telemetry.value.localIP = event.payload.localIP;
    telemetry.value.pitch = event.payload.pitch;
    telemetry.value.yaw = event.payload.yaw;
    telemetry.value.roll = event.payload.roll;
    telemetry.value.speed = event.payload.speed;
    telemetry.value.altitude = event.payload.altitude;
    telemetry.value.batteryLife = event.payload.batteryLife;
    telemetry.value.currentPosition = event.payload.currentPosition;
    telemetry.value.lastUpdated = event.payload.lastUpdated;
    telemetry.value.fireFound = event.payload.fireFound;
    telemetry.value.fireCoordinate = event.payload.fireCoordinate;
  });
  // setInterval(async () => {
  //   console.log("Polling telemetry...");
  //   await getTelemetry();
  // }, 1000);
});
</script>

<template>
  <div>
    <h3>Most Recent Stage:</h3>
    <p><strong>Stage ID:</strong> {{ mostRecentStage.stageId }}</p>
    <p><strong>Stage Name:</strong> {{ mostRecentStage.stageName }}</p>
    <p><strong>Vehicle Name:</strong> {{ mostRecentStage.vehicleName }}</p>
    <button @click="getMostRecentStage">Add New Stage</button> 

    <h3>Telemetry Data:</h3>
    <p><strong>Local IP:</strong> {{ telemetry.localIP }}</p>
    <p><strong>Pitch:</strong> {{ telemetry.pitch }}</p>
    <p><strong>Yaw:</strong> {{ telemetry.yaw }}</p>
    <p><strong>Roll:</strong> {{ telemetry.roll }}</p>
    <p><strong>Speed:</strong> {{ telemetry.speed }} km/h</p>
    <p><strong>Altitude:</strong> {{ telemetry.altitude }} m</p>
    <p><strong>Battery Life:</strong> {{ telemetry.batteryLife }}%</p>
    <p>
      <strong>Position:</strong> {{ telemetry.currentPosition.latitude }},
      {{ telemetry.currentPosition.longitude }}
    </p>
    <p><strong>Last Updated:</strong> {{ telemetry.lastUpdated }}</p>
    <p><strong>Fire Found:</strong> {{ telemetry.fireFound ? "Yes" : "No" }}</p>
    <p>
      <strong>Fire Coordinates:</strong> {{ telemetry.fireCoordinate.latitude }},
      {{ telemetry.fireCoordinate.longitude }}
    </p>
  </div>
</template>
