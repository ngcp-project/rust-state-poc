<template>
  <div class="container">
    Set managed state on backend <input type="text" v-model="value" />
    <button @click="callBackend">Call Backend code</button>

    <br />
    Current State (uppercase): {{ state }}
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted, onBeforeUnmount } from "vue";
import { createTauRPCProxy } from "./lib/ipc";

export default defineComponent({
  setup() {
    const value = ref("");
    const state = ref("");
    let unlisten: Array<() => void> = [];
    let taurpc: Awaited<ReturnType<typeof createTauRPCProxy>>;

    const callBackend = async () => {
      await taurpc.update_state(value.value);
      await taurpc.get_window();
      await taurpc.method_with_alias();
      await taurpc.multiple_args([], "test");
      await taurpc.get_app_handle();

      try {
        const res = await taurpc.test_result({
          welcome_message: "test message",
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

    return {
      value,
      state,
      callBackend
    };
  }
});
</script>

<style scoped></style>
