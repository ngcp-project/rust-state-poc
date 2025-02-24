<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Count from "./components/Count.vue";
import Stage from "./components/Stage.vue";
import SharedCount from "./components/SharedCount.vue";
import LocalCount from "./components/LocalCount.vue";
import Form from "./components/Form.vue";
import CreateMission from "./components/CreateMission.vue";
</script>

<template>
  <div class="container">
    <div>
      <CreateMission />
    </div>
    <div>
      <SharedCount />
    </div>
    <div>
      <LocalCount />
    </div>
    <div>
      <Form />
    </div>
    <!-- <GenericCount /> -->
    <Stage />
  </div>
</template>

<style scoped></style>
