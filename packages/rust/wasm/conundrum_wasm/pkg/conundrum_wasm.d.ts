/* tslint:disable */
/* eslint-disable */

/**
 * `Decimal` represents a 128 bit representation of a fixed-precision decimal number.
 * The finite set of values of type `Decimal` are of the form m / 10<sup>e</sup>,
 * where m is an integer such that -2<sup>96</sup> < m < 2<sup>96</sup>, and e is an integer
 * between 0 and 28 inclusive.
 */
export class Decimal {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Returns a new `Decimal` object instance by converting a primitive number.
     *
     * Returns `undefined` in JavaScript if the value cannot be represented as a `Decimal`
     * (e.g. `NaN`, `Infinity`, or `-Infinity`).
     */
    static fromNumber(value: number): Decimal | undefined;
    /**
     * Returns a new `Decimal` object instance by parsing a string representation.
     *
     * Returns `undefined` in JavaScript if the string cannot be parsed as a valid `Decimal`.
     */
    static fromString(value: string): Decimal | undefined;
    /**
     * Returns the value of this `Decimal` converted to a primitive number.
     *
     * # Note
     * At the time of writing, the conversion from `Decimal` to `f64` cannot fail. To guard
     * against future implementation changes, `f64::NAN` is returned as a fallback value.
     */
    toNumber(): number;
    /**
     * Returns the string representation of this `Decimal`.
     *
     * This intentionally overrides the default JS `toString()` so that string coercion
     * (e.g. template literals, `console.log`) produces the decimal representation.
     */
    toString(): string;
}

/**
 * Global context struct for KaTeX
 *
 * The KatexContext serves as the central registry for all KaTeX functionality,
 * containing mappings for functions, HTML/MathML builders, symbols, and
 * environments. This design enables runtime extensibility of LaTeX commands
 * without recompilation.
 */
export class KatexContext {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
}

/**
 * Main error type thrown by KaTeX functions when something has gone wrong.
 * This is used to distinguish internal errors from errors in the expression
 * that the user provided.
 */
export class ParseError {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
}

/**
 * Core settings structure for KaTeX rendering configuration.
 *
 * This struct contains all resolved configuration options that control
 * KaTeX's behavior during mathematical expression parsing and rendering.
 * Unlike the builder inputs, all fields have concrete values with no options.
 *
 * # LaTeX/KaTeX Context
 * These settings correspond to LaTeX document and package options that affect
 * mathematical typesetting. KaTeX uses this structure to maintain consistent
 * rendering behavior across different expressions and contexts.
 *
 * # Cross-references
 * - See [`Settings::builder`] for ergonomic construction of settings.
 * - Related to [`OutputFormat`], [`StrictSetting`], and [`TrustSetting`].
 * - Methods provide validation and utility functions.
 */
export class Settings {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Whether `\color` commands affect surrounding text color.
     *
     * When `true`, color commands modify text color. When `false`,
     * they only affect mathematical content.
     */
    color_is_text_color: boolean;
    /**
     * Color value for mathematical content.
     *
     * CSS color value used for rendering mathematical expressions.
     */
    get color(): string | undefined;
    /**
     * Color value for mathematical content.
     *
     * CSS color value used for rendering mathematical expressions.
     */
    set color(value: string | null | undefined);
    /**
     * Whether mathematical expressions are rendered in display (block) mode.
     *
     * When `true`, equations are centered and displayed on separate lines
     * with larger fonts. When `false`, equations are rendered inline.
     */
    display_mode: boolean;
    /**
     * CSS color value used for rendering error messages.
     *
     * Applied to error text when `throw_on_error` is `false`.
     */
    error_color: string;
    /**
     * Whether equations are flushed to the left margin.
     *
     * When `true`, equations are left-aligned instead of centered.
     */
    fleqn: boolean;
    /**
     * Whether settings persist globally across render calls.
     *
     * When `true`, settings remain active for subsequent expressions.
     */
    global_group: boolean;
    /**
     * Whether equation numbers are placed on the left side.
     *
     * Controls the positioning of equation numbers in numbered environments.
     */
    leqno: boolean;
    /**
     * Maximum limit for macro expansion iterations.
     *
     * Prevents infinite loops in macro expansion.
     */
    max_expand: number;
    /**
     * Maximum allowed size for rendered expressions.
     *
     * Prevents excessive memory usage from very large expressions. In points.
     */
    max_size: number;
    /**
     * Minimum thickness for rendered rules (lines).
     *
     * Prevents lines from becoming too thin to be visible. In points.
     */
    min_rule_thickness: number;
    /**
     * Size multiplier for scaling rendered expressions.
     *
     * Controls the overall size scaling factor for mathematical expressions.
     */
    size_multiplier: number;
    /**
     * Whether parsing/rendering errors should throw exceptions.
     *
     * When `true`, errors cause panics. When `false`, errors are rendered
     * as colored text in the output.
     */
    throw_on_error: boolean;
}

export function compile_cdrm(cdrm: string, ui_params: any, modifiers: any, trusted: boolean): any;

export function search_conundrum_emojis(query: string, page: number, per_page: number): any;

export function search_conundrum_emojis_in_docs_container(query: string, container_id: string, page: number, per_page: number): any;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly compile_cdrm: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
    readonly search_conundrum_emojis: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly search_conundrum_emojis_in_docs_container: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
    readonly ffi_conundrum_rust_future_cancel_f32: (a: bigint) => void;
    readonly ffi_conundrum_rust_future_complete_f32: (a: bigint, b: number) => number;
    readonly ffi_conundrum_rust_future_complete_f64: (a: bigint, b: number) => number;
    readonly ffi_conundrum_rust_future_complete_i16: (a: bigint, b: number) => number;
    readonly ffi_conundrum_rust_future_complete_i32: (a: bigint, b: number) => number;
    readonly ffi_conundrum_rust_future_complete_i64: (a: bigint, b: number) => bigint;
    readonly ffi_conundrum_rust_future_complete_i8: (a: bigint, b: number) => number;
    readonly ffi_conundrum_rust_future_complete_rust_buffer: (a: number, b: bigint, c: number) => void;
    readonly ffi_conundrum_rust_future_complete_u16: (a: bigint, b: number) => number;
    readonly ffi_conundrum_rust_future_complete_u8: (a: bigint, b: number) => number;
    readonly ffi_conundrum_rust_future_complete_void: (a: bigint, b: number) => void;
    readonly ffi_conundrum_rust_future_free_f32: (a: bigint) => void;
    readonly ffi_conundrum_rust_future_poll_f32: (a: bigint, b: number, c: bigint) => void;
    readonly ffi_conundrum_rustbuffer_alloc: (a: number, b: bigint, c: number) => void;
    readonly ffi_conundrum_rustbuffer_free: (a: number, b: number) => void;
    readonly ffi_conundrum_rustbuffer_from_bytes: (a: number, b: number, c: number) => void;
    readonly ffi_conundrum_rustbuffer_reserve: (a: number, b: number, c: bigint, d: number) => void;
    readonly ffi_conundrum_uniffi_contract_version: () => number;
    readonly __wbg_get_settings_color: (a: number, b: number) => void;
    readonly __wbg_get_settings_color_is_text_color: (a: number) => number;
    readonly __wbg_get_settings_display_mode: (a: number) => number;
    readonly __wbg_get_settings_error_color: (a: number, b: number) => void;
    readonly __wbg_get_settings_fleqn: (a: number) => number;
    readonly __wbg_get_settings_global_group: (a: number) => number;
    readonly __wbg_get_settings_leqno: (a: number) => number;
    readonly __wbg_get_settings_max_expand: (a: number) => number;
    readonly __wbg_get_settings_max_size: (a: number) => number;
    readonly __wbg_get_settings_min_rule_thickness: (a: number) => number;
    readonly __wbg_get_settings_size_multiplier: (a: number) => number;
    readonly __wbg_get_settings_throw_on_error: (a: number) => number;
    readonly __wbg_katexcontext_free: (a: number, b: number) => void;
    readonly __wbg_parseerror_free: (a: number, b: number) => void;
    readonly __wbg_set_settings_color: (a: number, b: number, c: number) => void;
    readonly __wbg_set_settings_color_is_text_color: (a: number, b: number) => void;
    readonly __wbg_set_settings_display_mode: (a: number, b: number) => void;
    readonly __wbg_set_settings_error_color: (a: number, b: number, c: number) => void;
    readonly __wbg_set_settings_fleqn: (a: number, b: number) => void;
    readonly __wbg_set_settings_global_group: (a: number, b: number) => void;
    readonly __wbg_set_settings_leqno: (a: number, b: number) => void;
    readonly __wbg_set_settings_max_expand: (a: number, b: number) => void;
    readonly __wbg_set_settings_max_size: (a: number, b: number) => void;
    readonly __wbg_set_settings_min_rule_thickness: (a: number, b: number) => void;
    readonly __wbg_set_settings_size_multiplier: (a: number, b: number) => void;
    readonly __wbg_set_settings_throw_on_error: (a: number, b: number) => void;
    readonly __wbg_settings_free: (a: number, b: number) => void;
    readonly qcms_profile_precache_output_transform: (a: number) => void;
    readonly lut_inverse_interp16: (a: number, b: number, c: number) => number;
    readonly qcms_transform_data_bgra_out_lut_precache: (a: number, b: number, c: number, d: number) => void;
    readonly qcms_transform_data_rgba_out_lut_precache: (a: number, b: number, c: number, d: number) => void;
    readonly qcms_transform_data_bgra_out_lut: (a: number, b: number, c: number, d: number) => void;
    readonly qcms_transform_data_rgba_out_lut: (a: number, b: number, c: number, d: number) => void;
    readonly qcms_transform_data_rgb_out_lut_precache: (a: number, b: number, c: number, d: number) => void;
    readonly qcms_transform_data_rgb_out_lut: (a: number, b: number, c: number, d: number) => void;
    readonly lut_interp_linear16: (a: number, b: number, c: number) => number;
    readonly qcms_profile_is_bogus: (a: number) => number;
    readonly qcms_transform_release: (a: number) => void;
    readonly qcms_white_point_sRGB: (a: number) => void;
    readonly __wbg_decimal_free: (a: number, b: number) => void;
    readonly decimal_fromNumber: (a: number) => number;
    readonly decimal_fromString: (a: number, b: number) => number;
    readonly decimal_toNumber: (a: number) => number;
    readonly decimal_toString: (a: number, b: number) => void;
    readonly qcms_enable_iccv4: () => void;
    readonly ffi_conundrum_rust_future_cancel_pointer: (a: bigint) => void;
    readonly ffi_conundrum_rust_future_cancel_i8: (a: bigint) => void;
    readonly ffi_conundrum_rust_future_cancel_f64: (a: bigint) => void;
    readonly ffi_conundrum_rust_future_cancel_u8: (a: bigint) => void;
    readonly ffi_conundrum_rust_future_cancel_i32: (a: bigint) => void;
    readonly ffi_conundrum_rust_future_complete_pointer: (a: bigint, b: number) => number;
    readonly ffi_conundrum_rust_future_cancel_u32: (a: bigint) => void;
    readonly ffi_conundrum_rust_future_complete_u32: (a: bigint, b: number) => number;
    readonly ffi_conundrum_rust_future_cancel_i16: (a: bigint) => void;
    readonly ffi_conundrum_rust_future_cancel_u16: (a: bigint) => void;
    readonly ffi_conundrum_rust_future_cancel_void: (a: bigint) => void;
    readonly ffi_conundrum_rust_future_cancel_i64: (a: bigint) => void;
    readonly ffi_conundrum_rust_future_cancel_u64: (a: bigint) => void;
    readonly ffi_conundrum_rust_future_complete_u64: (a: bigint, b: number) => bigint;
    readonly ffi_conundrum_rust_future_cancel_rust_buffer: (a: bigint) => void;
    readonly ffi_conundrum_rust_future_free_rust_buffer: (a: bigint) => void;
    readonly ffi_conundrum_rust_future_poll_rust_buffer: (a: bigint, b: number, c: bigint) => void;
    readonly ffi_conundrum_rust_future_free_pointer: (a: bigint) => void;
    readonly ffi_conundrum_rust_future_poll_pointer: (a: bigint, b: number, c: bigint) => void;
    readonly ffi_conundrum_rust_future_free_i8: (a: bigint) => void;
    readonly ffi_conundrum_rust_future_poll_i8: (a: bigint, b: number, c: bigint) => void;
    readonly ffi_conundrum_rust_future_free_f64: (a: bigint) => void;
    readonly ffi_conundrum_rust_future_poll_f64: (a: bigint, b: number, c: bigint) => void;
    readonly ffi_conundrum_rust_future_free_u8: (a: bigint) => void;
    readonly ffi_conundrum_rust_future_poll_u8: (a: bigint, b: number, c: bigint) => void;
    readonly ffi_conundrum_rust_future_free_i32: (a: bigint) => void;
    readonly ffi_conundrum_rust_future_poll_i32: (a: bigint, b: number, c: bigint) => void;
    readonly ffi_conundrum_rust_future_free_u32: (a: bigint) => void;
    readonly ffi_conundrum_rust_future_poll_u32: (a: bigint, b: number, c: bigint) => void;
    readonly ffi_conundrum_rust_future_free_i16: (a: bigint) => void;
    readonly ffi_conundrum_rust_future_poll_i16: (a: bigint, b: number, c: bigint) => void;
    readonly ffi_conundrum_rust_future_free_u16: (a: bigint) => void;
    readonly ffi_conundrum_rust_future_poll_u16: (a: bigint, b: number, c: bigint) => void;
    readonly ffi_conundrum_rust_future_free_void: (a: bigint) => void;
    readonly ffi_conundrum_rust_future_poll_void: (a: bigint, b: number, c: bigint) => void;
    readonly ffi_conundrum_rust_future_free_i64: (a: bigint) => void;
    readonly ffi_conundrum_rust_future_poll_i64: (a: bigint, b: number, c: bigint) => void;
    readonly ffi_conundrum_rust_future_free_u64: (a: bigint) => void;
    readonly ffi_conundrum_rust_future_poll_u64: (a: bigint, b: number, c: bigint) => void;
    readonly __wbindgen_export: (a: number, b: number) => number;
    readonly __wbindgen_export2: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_export3: (a: number) => void;
    readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
    readonly __wbindgen_export4: (a: number, b: number, c: number) => void;
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
