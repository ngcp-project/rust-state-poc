import { createStore } from "zustand/vanilla";

// Define Zustand Store types
interface CountState {
  count: number;
  increaseCount: () => void;
}

const useSharedCountStore = createStore<CountState>((set) => ({
  // Initial State
  count: 0, 
  // State Method
  increaseCount: () => set((state) => ({ count: state.count + 1 }))
}));


// Destructure and Export Zustand Store Methods
export const { getState, setState, subscribe } = useSharedCountStore;

// Default export containing Zustand Store
export default useSharedCountStore;