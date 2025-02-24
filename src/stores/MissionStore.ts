import { createStore } from "zustand/vanilla";
import { createTauRPCProxy, MissionStateStruct, MissionDataStruct } from "../lib/bindings";

const taurpc = await createTauRPCProxy();

// MissionStateStruct provides the state variables, so extend it with new methods
interface MissionState extends MissionStateStruct {
  nextStep: () => Promise<null>;
  previousStep: () => Promise<null>;
  updateMissionData: (missionData: MissionDataStruct) => Promise<null>;
  submitMission: () => Promise<null>;
  reset: () => Promise<null>;
}

const initialState: MissionStateStruct = await taurpc.mission.get_default_data();

const useMissionStore = createStore<MissionState>(() => ({
  // Destructure the initial state to populate it with the state variables
  ...initialState,

  nextStep: async () => await taurpc.mission.next_step(),
  previousStep: async () => await taurpc.mission.previous_step(),
  updateMissionData: async (missionData: MissionDataStruct) =>
    await taurpc.mission.update_mission_data(missionData),

  submitMission: async () => {
    console.log("Mission submitted: " + JSON.stringify(taurpc.mission.get_data()));
    return await taurpc.mission.submit_mission();
  },
  reset: async () => await taurpc.mission.reset(),
}));

// Get current mission state from tauri
// Use event listener to initialize on reload
// initialState may be out of date
taurpc.mission.get_data().then((data: MissionStateStruct) => {
  console.log("Mission Data Initialized", data);
  useMissionStore.setState({ ...data });
});

// Update mission state when tauri emits state update
taurpc.mission.on_updated.on((new_data: MissionStateStruct) => {
  console.log("Mission data updated", new_data);
  useMissionStore.setState({ ...new_data });
});

export default useMissionStore;
export const { getState, setState, subscribe } = useMissionStore;
