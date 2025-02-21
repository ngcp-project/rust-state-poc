<template>
    <!-- Counter Progress -->
    <div>
        Current Step: {{ currentStep }} of {{ totalSteps }}
    </div>
    <!-- Step 1 -->
    <div v-if="currentStep === 1">
        <label for="name">Name:</label>
        <input
            type="text"
            id="name"
            v-model.number="formData.name"
            @change="updateForm($event)"
            required
        />
        <label for="email">Email:</label>
        <input
            type="email"
            id="email"
            v-model.number="formData.email"
            @change="updateForm($event)"
            required
        />
    </div>
    <!-- Step 2 -->
    <div v-if="currentStep === 2">
        <label for="phone">Phone:</label>
        <input
            type="tel"
            id="phone"
            v-model.number="formData.phone"
            @change="updateForm($event)"
            required
        />
        <button @click="submitForm">Submit</button>
    </div>

    <!-- Submission -->
    <div v-if="isSubmitted">
        <h3>Form Submitted Successfully!</h3>
        <p>Your data has been received.</p>
        <button @click="reset">Reset Form</button>
    </div>

    <!-- Buttons -->
    <div v-if="!isSubmitted">
        <button @click="previousStep">Previous</button>
        <button @click="nextStep">Next</button>
        <button @click="reset">Reset Form</button>
    </div>
    <div>
        
    </div>
</template>

<script setup lang="ts">
import {getState, subscribe} from "../stores/formStore";
import { onMounted, ref } from "vue";
import { FormDataStruct } from "../lib/bindings";


// Initialize refs for variables used in template
// Note: that we have to use rust getState() methods and variables
const currentStep = ref(getState().current_step)
const totalSteps = ref(getState().total_steps)
const formData = ref(getState().form_data)
const isSubmitted = ref(getState().is_submitted)

// Create a handler method for updating form data
// Must use async since all tauri methods are async
const nextStep = async () => await getState().nextStep()
const previousStep = async () => await getState().previousStep()
const reset = async () => await getState().reset()
const submitForm = async () => await getState().submitForm()

const updateForm = async (e: Event) => { 
    const target = e.target as HTMLInputElement
    console.log(target.id, target.value)
    
    // Update the local zustand store    
    // Use FormDataStruct keys (generated from tauri) to update the nested object
    formData.value[target.id as keyof FormDataStruct] = target.value
    
    // Call update form method on tauri passing the updated form data
    // while retaining other existing form data properties 
    await getState().updateForm({...getState().form_data, ...formData.value})

}

// When the component is mounted/loaded in DOM
onMounted(() => {
    // Ref values for each variable are subscribed to zustand store changes
    // in order to synchronize refs with the tauri store
    subscribe(() => {
        currentStep.value = getState().current_step;
        totalSteps.value = getState().total_steps;
        formData.value = getState().form_data;
        isSubmitted.value = getState().is_submitted;
    })
})

</script>