 // This file has been generated by Specta. DO NOT EDIT.

export type CounterStore = { count: number }

export type FormDataStruct = { name: string; email: string; phone: string }

export type FormStateStruct = { current_step: number; total_steps: number; form_data: FormDataStruct; is_submitted: boolean }

export type MissionDataStruct = { mission_name: string; keep_out_zone: string[]; keep_in_zone: string[]; status: MissionStatus }

export type MissionStateStruct = { current_step: number; total_steps: number; mission_data: MissionDataStruct; is_submitted: boolean }

export type MissionStatus = "Active" | "Inactive" | "Complete" | "Failed"

export type TauRpcCounterApiInputTypes = { proc_name: "increase"; input_type: null } | { proc_name: "get_data"; input_type: null } | { proc_name: "get_default_data"; input_type: null } | { proc_name: "on_updated"; input_type: { __taurpc_type: CounterStore } }

export type TauRpcCounterApiOutputTypes = { proc_name: "increase"; output_type: null } | { proc_name: "get_data"; output_type: CounterStore } | { proc_name: "get_default_data"; output_type: CounterStore } | { proc_name: "on_updated"; output_type: null }

export type TauRpcFormApiInputTypes = { proc_name: "next_step"; input_type: null } | { proc_name: "previous_step"; input_type: null } | { proc_name: "reset"; input_type: null } | { proc_name: "update_form"; input_type: { __taurpc_type: FormDataStruct } } | { proc_name: "submit_form"; input_type: null } | { proc_name: "get_default_data"; input_type: null } | { proc_name: "get_data"; input_type: null } | { proc_name: "on_updated"; input_type: { __taurpc_type: FormStateStruct } }

export type TauRpcFormApiOutputTypes = { proc_name: "next_step"; output_type: null } | { proc_name: "previous_step"; output_type: null } | { proc_name: "reset"; output_type: null } | { proc_name: "update_form"; output_type: null } | { proc_name: "submit_form"; output_type: null } | { proc_name: "get_default_data"; output_type: FormStateStruct } | { proc_name: "get_data"; output_type: FormStateStruct } | { proc_name: "on_updated"; output_type: null }

export type TauRpcMissionApiInputTypes = { proc_name: "next_step"; input_type: null } | { proc_name: "previous_step"; input_type: null } | { proc_name: "reset"; input_type: null } | { proc_name: "update_mission_data"; input_type: { __taurpc_type: MissionDataStruct } } | { proc_name: "append_keep_in_out_zone_coords"; input_type: [string, string] } | { proc_name: "submit_mission"; input_type: null } | { proc_name: "get_default_data"; input_type: null } | { proc_name: "get_data"; input_type: null } | { proc_name: "on_updated"; input_type: { __taurpc_type: MissionStateStruct } }

export type TauRpcMissionApiOutputTypes = { proc_name: "next_step"; output_type: null } | { proc_name: "previous_step"; output_type: null } | { proc_name: "reset"; output_type: null } | { proc_name: "update_mission_data"; output_type: null } | { proc_name: "append_keep_in_out_zone_coords"; output_type: null } | { proc_name: "submit_mission"; output_type: null } | { proc_name: "get_default_data"; output_type: MissionStateStruct } | { proc_name: "get_data"; output_type: MissionStateStruct } | { proc_name: "on_updated"; output_type: null }

const ARGS_MAP = {"mission":"{\"next_step\":[],\"append_keep_in_out_zone_coords\":[\"keep_in_zone\",\"keep_out_zone\"],\"update_mission_data\":[\"mission_data\"],\"previous_step\":[],\"reset\":[],\"submit_mission\":[],\"on_updated\":[\"new_data\"],\"get_default_data\":[],\"get_data\":[]}","form":"{\"reset\":[],\"update_form\":[\"form_data\"],\"get_data\":[],\"submit_form\":[],\"previous_step\":[],\"next_step\":[],\"get_default_data\":[],\"on_updated\":[\"new_data\"]}","counter":"{\"get_data\":[],\"increase\":[],\"get_default_data\":[],\"on_updated\":[\"new_data\"]}"}
import { createTauRPCProxy as createProxy } from "taurpc"

export const createTauRPCProxy = () => createProxy<Router>(ARGS_MAP)

type Router = {
	'form': [TauRpcFormApiInputTypes, TauRpcFormApiOutputTypes],
	'counter': [TauRpcCounterApiInputTypes, TauRpcCounterApiOutputTypes],
	'mission': [TauRpcMissionApiInputTypes, TauRpcMissionApiOutputTypes],
}