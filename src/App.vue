<template>
  <div class="container">
    Count: {{ appData?.count ?? 0 }}

    <button @click="increaseCount">Increase count</button>
  </div>
</template>

<script lang="ts" setup>
import { ref, onMounted, onBeforeUnmount } from "vue";
import { createTauRPCProxy } from "./lib/ipc";
import { AppData } from "./lib/bindings";

const appData = ref<AppData | null>(null);

const value = ref("");
const state = ref("");
let unlisten: Array<() => void> = [];
let taurpc: Awaited<ReturnType<typeof createTauRPCProxy>>;

const increaseCount = async () => {
  await taurpc.increase_count();
};

const callBackend = async () => {
  await taurpc.update_state(value.value);
  await taurpc.get_window();
  await taurpc.method_with_alias();
  await taurpc.multiple_args([], "test");
  await taurpc.get_app_handle();

  try {
    const res = await taurpc.test_result({
      state: "test message",
      count: 10
    });
    console.log({ res });
  } catch (error) {
    console.error({ error });
    // Handle error
  }
};

onMounted(async () => {
  taurpc = await createTauRPCProxy();
  taurpc.get_app_data().then((data) => {
    appData.value = data;
  });
  unlisten.push(
    taurpc.events.data_changed.on((new_data) => {
      console.log("app data updated", new_data);
      appData.value = { ...appData.value, ...new_data };
    })
  );
  unlisten.push(
    taurpc.events.vec_test.on((new_state) => {
      console.log("state updated", new_state);
    })
  );
  unlisten.push(
    taurpc.events.state_changed.on((val) => {
      state.value = val;
    })
  );
  unlisten.push(
    taurpc.events.multiple_args.on((arg1, arg2) => {
      console.log(arg1, arg2);
    })
  );
});

onBeforeUnmount(() => {
  unlisten.forEach((fn) => fn());
});
</script>

<style scoped></style>
