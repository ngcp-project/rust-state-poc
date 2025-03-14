 // This file has been generated by Specta. DO NOT EDIT.

export type CounterStore = { count: number }

export type FormDataStruct = { name: string; email: string; phone: string }

export type FormStateStruct = { current_step: number; total_steps: number; form_data: FormDataStruct; is_submitted: boolean }

export type MissionInfoStruct = { current_mission: MissionStruct; mission_form_state: MissionStruct; stages: string[]; current_stage_id: number; is_submitted: boolean }

export type MissionStageStruct = { stage_name: string; vehicle_name: string; search_area: string; target_coordinate: string }

export type MissionStatus = "Active" | "Inactive" | "Complete" | "Failed"

export type MissionStruct = { mission_name: string; keep_in_zone: string; keep_out_zone: string; status: MissionStatus }

export type TauRpcCounterApiInputTypes = { proc_name: "increase"; input_type: null } | { proc_name: "get_data"; input_type: null } | { proc_name: "get_default_data"; input_type: null } | { proc_name: "on_updated"; input_type: { __taurpc_type: CounterStore } }

export type TauRpcCounterApiOutputTypes = { proc_name: "increase"; output_type: null } | { proc_name: "get_data"; output_type: CounterStore } | { proc_name: "get_default_data"; output_type: CounterStore } | { proc_name: "on_updated"; output_type: null }

export type TauRpcFormApiInputTypes = { proc_name: "next_step"; input_type: null } | { proc_name: "previous_step"; input_type: null } | { proc_name: "reset"; input_type: null } | { proc_name: "update_form"; input_type: { __taurpc_type: FormDataStruct } } | { proc_name: "submit_form"; input_type: null } | { proc_name: "get_default_data"; input_type: null } | { proc_name: "get_data"; input_type: null } | { proc_name: "on_updated"; input_type: { __taurpc_type: FormStateStruct } }

export type TauRpcFormApiOutputTypes = { proc_name: "next_step"; output_type: null } | { proc_name: "previous_step"; output_type: null } | { proc_name: "reset"; output_type: null } | { proc_name: "update_form"; output_type: null } | { proc_name: "submit_form"; output_type: null } | { proc_name: "get_default_data"; output_type: FormStateStruct } | { proc_name: "get_data"; output_type: FormStateStruct } | { proc_name: "on_updated"; output_type: null }

export type TauRpcMissionApiInputTypes = { proc_name: "transition_next_stage"; input_type: null } | { proc_name: "get_default_data"; input_type: null } | { proc_name: "get_data"; input_type: null } | { proc_name: "reset"; input_type: null } | { proc_name: "submit_mission"; input_type: null } | { proc_name: "update_mission_data"; input_type: { __taurpc_type: MissionInfoStruct } } | { proc_name: "on_updated"; input_type: { __taurpc_type: MissionInfoStruct } }

export type TauRpcMissionApiOutputTypes = { proc_name: "transition_next_stage"; output_type: null } | { proc_name: "get_default_data"; output_type: MissionInfoStruct } | { proc_name: "get_data"; output_type: MissionInfoStruct } | { proc_name: "reset"; output_type: null } | { proc_name: "submit_mission"; output_type: null } | { proc_name: "update_mission_data"; output_type: null } | { proc_name: "on_updated"; output_type: null }

export type TestMissionStruct = { stages: string[]; current_stage_id: number }

export type VehicleStruct = { vehicle_name: string; current_stage_id: string; stages_list: MissionStageStruct[] }

const ARGS_MAP = {"mission":"{\"transition_next_stage\":[],\"get_data\":[],\"submit_mission\":[],\"update_mission_data\":[\"mission_data\"],\"on_updated\":[\"new_data\"],\"reset\":[],\"get_default_data\":[]}","form":"{\"get_default_data\":[],\"update_form\":[\"form_data\"],\"on_updated\":[\"new_data\"],\"reset\":[],\"get_data\":[],\"submit_form\":[],\"next_step\":[],\"previous_step\":[]}","counter":"{\"get_data\":[],\"on_updated\":[\"new_data\"],\"increase\":[],\"get_default_data\":[]}"}
import { createTauRPCProxy as createProxy } from "taurpc"

export const createTauRPCProxy = () => createProxy<Router>(ARGS_MAP)

type Router = {
	'form': [TauRpcFormApiInputTypes, TauRpcFormApiOutputTypes],
	'counter': [TauRpcCounterApiInputTypes, TauRpcCounterApiOutputTypes],
	'mission': [TauRpcMissionApiInputTypes, TauRpcMissionApiOutputTypes],
}