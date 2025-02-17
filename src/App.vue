<template>
  <div class="container">
    <div>
      <SharedCount />
    </div>

    <div>
      <LocalCount />
    </div>
    <div>
      <Form/>
    </div>
    <!-- <GenericCount /> -->
  </div>
</template>

<script lang="ts" setup>
import SharedCount from './components/SharedCount.vue';
import LocalCount from './components/LocalCount.vue';
import Form from './components/Form.vue';

</script>

<style scoped></style>
