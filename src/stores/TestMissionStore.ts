import { createStore } from "zustand/vanilla";
import { TestMissionStruct, createTauRPCProxy } from "../lib/bindings";


// Create taurpc proxy
const taurpc = await createTauRPCProxy();

// // Define Zustand Store types
interface TestMissionState extends TestMissionStruct {
  transitionNextStage: () => void;
}


const initialState: TestMissionState = await taurpc.mission.get_default_data();

const MissionStore = createStore<TestMissionState>(() => ({
  // Initial State
  ...initialState,
  // State Method
  // Use async functions for taurpc calls
  transitionNextStage: async () => {
    await taurpc.mission.transition_next_stage();
  }
}));

// Destructure and Export Zustand Store Methods
export const { getState, setState, subscribe } = MissionStore;

// Default export containing Zustand Store
export default MissionStore;

// Initialize count with taurpc.get_app_data
taurpc.mission.get_data().then((data: TestMissionStruct) => {
  setState({...data});
});

// Update count when taurpc.events.data_changed is triggered
taurpc.mission.on_updated.on((new_data: TestMissionStruct) => {
  console.log("app data updated", new_data);
  MissionStore.setState({...new_data});
});


