import { createStore } from "zustand/vanilla";
import { CounterStore, createTauRPCProxy } from "../lib/bindings";


// Create taurpc proxy
const taurpc = await createTauRPCProxy();

// Define Zustand Store types
interface CountState extends CounterStore {
  increaseCount: () => void;
}


const initialState: CounterStore = await taurpc.counter.get_default_data();

const useSharedCountStore = createStore<CountState>(() => ({
  // Initial State
  ...initialState,
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
taurpc.counter.get_data().then((data: CounterStore) => {
  setState({...data});
});

// Update count when taurpc.events.data_changed is triggered
taurpc.counter.on_updated.on((new_data: CounterStore) => {
  console.log("app data updated", new_data);
  useSharedCountStore.setState({...new_data});
});


