import { createStore } from "zustand/vanilla";
import { createTauRPCProxy } from "../bindings";

interface CountState {
  count: number;
  increaseCount: () => void;
}

const taurpc = await createTauRPCProxy();

const useCountStore = createStore<CountState>(() => ({
  count: 0,
  increaseCount: async () => {
    await taurpc.increase_count();
  }
}));

export const { getState, setState, subscribe } = useCountStore;

// Initialize count with taurpc.get_app_data
taurpc.get_app_data().then((data: { count: number }) => {
  setState({ count: data.count });
});

// Update count when taurpc.events.data_changed is triggered
taurpc.events.data_changed.on((new_data: { count: number }) => {
  console.log("app data updated", new_data);
  useCountStore.setState({ count: new_data.count });
});

export default useCountStore;
