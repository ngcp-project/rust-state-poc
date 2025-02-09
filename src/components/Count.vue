<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const vehicleName = ref("Vehicle 1");
const vehicleData = ref("");

const fetchVehicleData = async () => {
  try {
    console.log("Sending request with:", { request: { vehicleName: vehicleName.value } });

    vehicleData.value = await invoke("get_vehicle_data", {
      request: {
        vehicleName: vehicleName.value
      }
    });

    console.log("Vehicle Data:", vehicleData.value);
  } catch (error) {
    console.error("Error details:", {
      message: error.message,
      error: error
    });
  }
};
</script>

<template>
  <div>
    <button @click="fetchVehicleData">Get Vehicle Data</button>
    <p>Result: {{ JSON.stringify(vehicleData) }}</p>
  </div>
</template>
