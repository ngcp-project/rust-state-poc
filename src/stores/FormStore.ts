import { createStore } from "zustand/vanilla";
import { createTauRPCProxy, FormStateStruct, FormDataStruct } from "../lib/bindings";


const taurpc = await createTauRPCProxy();

// FormStateStruct provides the state variables so extend it with new methods
interface FormState extends FormStateStruct {
  nextStep: () => Promise<null>;
  previousStep: () => Promise<null>;
  updateForm: (formData: FormDataStruct) => Promise<null>;
  submitForm: () => Promise<null>;
  reset: () => Promise<null>;
}

const initialState: FormStateStruct = await taurpc.form.get_default_data();


const useFormStore = createStore<FormState>(() => ({
  // Destructure the initial state to populate it with the state variables
  ...initialState,

  nextStep: async () => await taurpc.form.next_step(),
  previousStep: async () => await taurpc.form.previous_step(),
  updateForm: async (formData: FormDataStruct) => await taurpc.form.update_form(formData),
  
  submitForm: async () => {
    console.log("Form submitted: " + JSON.stringify(taurpc.form.get_data()))
    return await taurpc.form.submit_form();
  },
  reset: async () => await taurpc.form.reset(),
}))

// Get current form state from tauri
// use event listener to initialize on reload
// initialState may be out of date
taurpc.form.get_data().then((data: FormStateStruct) => {
  console.log("Form Data Initialized", data)
  useFormStore.setState({ ...data });
});

// Update form state when tauri emits state update
taurpc.form.on_updated.on((new_data: FormStateStruct) => {
  console.log("app data updated", new_data);
  useFormStore.setState({ ...new_data });
});

export default useFormStore;
export const { getState, setState, subscribe } = useFormStore;

/* 
Old form store that doesnt use tauri state management for reference

// Define the type for the form data
interface FormData {
  name: string;
  email: string;
  phone: string;
}

// Define the type for the store state
interface FormState {
  currentStep: number;
  totalSteps: number;
  formData: FormData;
  isSubmitted: boolean;

  submitForm: () => void;
  reset: () => void;
}

const initialState = { 
    currentStep: 1,
    totalSteps: 2,
    formData: { name: "", email: "", phone: "" },
    isSubmitted: false,
}


// Create the store with TypeScript types
const useFormStore = createStore<FormState>((set) => ({
  ...initialState,


  submitForm: () => {
    set((state) => {
    alert("Form submitted: " + JSON.stringify({ ...state }));

    return ({
      ...state,
      isSubmitted: true,
    })});
  },

  reset: () => {
    set({ ...initialState });
  },
}));
*/
