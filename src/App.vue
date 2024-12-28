<template>
  <div class="container">
    <div>
      <SharedCount />
    </div>

    <div>
      <LocalCount />
    </div>

    <!-- <GenericCount /> -->
  </div>
</template>

<script lang="ts" setup>
import SharedCount from './components/SharedCount.vue';
import LocalCount from './components/LocalCount.vue';
// import GenericCount from './components/GenericCount.vue';

</script>

<style scoped></style>
