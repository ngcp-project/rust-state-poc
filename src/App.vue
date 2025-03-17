<script setup lang="ts">
import SharedCount from "./components/SharedCount.vue";
import LocalCount from "./components/LocalCount.vue";
import Form from "./components/Form.vue";
import CreateMission from "./components/CreateMission.vue";
import TestMission from "./components/TestMission.vue";
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
    <TestMission />
  </div>
</template>

<style scoped></style>
