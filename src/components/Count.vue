<script setup>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

const vehicleName = ref("Vehicle 1");
const vehicleData = ref("");
const error = ref(null);

const fetchVehicleData = async () => {
  try {
    console.log("Invoking get_vehicle_data with vehicle_name: 'Vehicle 1'"); // Debugging statement
    vehicleData.value = await invoke("get_vehicle_data", { vehicle_name: vehicleName.value }); // Corrected key
    console.log("Fetched vehicle data:", vehicleData.value);
  } catch (err) {
    error.value = err.message;
    console.error("Error fetching vehicle data:", err);
  }
};

onMounted(fetchVehicleData);
</script>

<template>
  <div>
    <h2>Vehicle Data</h2>
    <div v-if="error">
      <p style="color: red">Error: {{ error }}</p>
    </div>
    <div v-else-if="vehicleData">
      <pre>{{ JSON.stringify(vehicleData) }}</pre>
    </div>
    <div v-else>
      <p>Loading...</p>
    </div>
  </div>
</template>
