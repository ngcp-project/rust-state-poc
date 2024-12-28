import { createStore } from "zustand/vanilla";
import { createTauRPCProxy } from "../lib/bindings";

// Define Zustand Store types
interface CountState {
  count: number;
  increaseCount: () => void;
}

// Create taurpc proxy
const taurpc = await createTauRPCProxy();

const useSharedCountStore = createStore<CountState>(() => ({
  // Initial State
  count: 0,
  // State Method
  // Use async functions for taurpc calls
  increaseCount: async () => {
    await taurpc.counter.increase();
  }
}));

// Destructure and Export Zustand Store Methods
export const { getState, setState, subscribe } = useSharedCountStore;

// Default export containing Zustand Store
export default useSharedCountStore;

// Initialize count with taurpc.get_app_data
taurpc.counter.get_data().then((data: { count: number }) => {
  setState({ count: data.count });
});

// Update count when taurpc.events.data_changed is triggered
taurpc.counter.on_updated.on((new_data: { count: number }) => {
  console.log("app data updated", new_data);
  useSharedCountStore.setState({ count: new_data.count });
});


