import { createStore } from "zustand/vanilla";
import { createTauRPCProxy, MissionInfoStruct } from "../lib/bindings";

const taurpc = await createTauRPCProxy();

// MissionStateStruct provides the state variables, so extend it with new methods
interface MissionState extends MissionInfoStruct {
  updateMissionData: (missionData: MissionInfoStruct) => Promise<null>;
  submitMission: () => Promise<null>;
  reset: () => Promise<null>;
}

const initialState: MissionInfoStruct = await taurpc.mission.get_default_data();

const useMissionStore = createStore<MissionState>(() => ({
  // Destructure the initial state to populate it with the state variables
  ...initialState,

  updateMissionData: async (missionData: MissionInfoStruct) =>
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
taurpc.mission.get_data().then((data: MissionInfoStruct) => {
  console.log("Mission Data Initialized", data);
  useMissionStore.setState({ ...data });
});

// Update mission state when tauri emits state update
taurpc.mission.on_updated.on((new_data: MissionInfoStruct) => {
  console.log("Mission data updated", new_data);
  useMissionStore.setState({ ...new_data });
});

export default useMissionStore;
export const { getState, setState, subscribe } = useMissionStore;
