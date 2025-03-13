/* tslint:disable */
/* eslint-disable */
export function initialize(): Promise<string>;
export function send_nostr_msg(msg: string): Promise<void>;
export function fetch_and_decrypt_local_messages(): Promise<any>;
export function save_image(file_name: string, file_bytes: Uint8Array): Promise<void>;
export function fetch_images(): Promise<any>;
export function fetch_nostr_events(from: string): Promise<any>;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly initialize: () => any;
  readonly send_nostr_msg: (a: number, b: number) => any;
  readonly fetch_and_decrypt_local_messages: () => any;
  readonly save_image: (a: number, b: number, c: number, d: number) => any;
  readonly fetch_images: () => any;
  readonly fetch_nostr_events: (a: number, b: number) => any;
  readonly rustsecp256k1_v0_10_0_context_create: (a: number) => number;
  readonly rustsecp256k1_v0_10_0_context_destroy: (a: number) => void;
  readonly rustsecp256k1_v0_10_0_default_illegal_callback_fn: (a: number, b: number) => void;
  readonly rustsecp256k1_v0_10_0_default_error_callback_fn: (a: number, b: number) => void;
  readonly ring_core_0_17_14__bn_mul_mont: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_6: WebAssembly.Table;
  readonly closure1095_externref_shim: (a: number, b: number, c: any) => void;
  readonly closure1177_externref_shim: (a: number, b: number, c: any) => void;
  readonly _dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hb8e4f6826d79d583: (a: number, b: number) => void;
  readonly closure2297_externref_shim: (a: number, b: number, c: any) => void;
  readonly closure2682_externref_shim: (a: number, b: number, c: any) => void;
  readonly closure4216_externref_shim_multivalue_shim: (a: number, b: number, c: any) => [number, number];
  readonly __externref_table_dealloc: (a: number) => void;
  readonly _dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h609308ea8ef84ba2: (a: number, b: number) => void;
  readonly closure4823_externref_shim: (a: number, b: number, c: any) => void;
  readonly closure5716_externref_shim: (a: number, b: number, c: any, d: any) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
