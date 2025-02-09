<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const vehicleName = ref("Vehicle 1");
const vehicleData = ref("");

const fetchVehicleData = async () => {
  try {
    vehicleData.value = await invoke("get_vehicle_data", { vehicle_name: vehicleName.value });
    console.log("Vehicle Data:", vehicleData.value);
  } catch (error) {
    console.error("Error fetching vehicle data:", error);
  }
};
</script>

<template>
  <div>
    <button @click="fetchVehicleData">Get Vehicle Data</button>
    <p>Result: {{ vehicleData }}</p>
  </div>
</template>
