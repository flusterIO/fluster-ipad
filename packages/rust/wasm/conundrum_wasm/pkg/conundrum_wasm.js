/* @ts-self-types="./conundrum_wasm.d.ts" */

/**
 * `Decimal` represents a 128 bit representation of a fixed-precision decimal number.
 * The finite set of values of type `Decimal` are of the form m / 10<sup>e</sup>,
 * where m is an integer such that -2<sup>96</sup> < m < 2<sup>96</sup>, and e is an integer
 * between 0 and 28 inclusive.
 */
export class Decimal {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Decimal.prototype);
        obj.__wbg_ptr = ptr;
        DecimalFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        DecimalFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_decimal_free(ptr, 0);
    }
    /**
     * Returns a new `Decimal` object instance by converting a primitive number.
     *
     * Returns `undefined` in JavaScript if the value cannot be represented as a `Decimal`
     * (e.g. `NaN`, `Infinity`, or `-Infinity`).
     * @param {number} value
     * @returns {Decimal | undefined}
     */
    static fromNumber(value) {
        const ret = wasm.decimal_fromNumber(value);
        return ret === 0 ? undefined : Decimal.__wrap(ret);
    }
    /**
     * Returns a new `Decimal` object instance by parsing a string representation.
     *
     * Returns `undefined` in JavaScript if the string cannot be parsed as a valid `Decimal`.
     * @param {string} value
     * @returns {Decimal | undefined}
     */
    static fromString(value) {
        const ptr0 = passStringToWasm0(value, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.decimal_fromString(ptr0, len0);
        return ret === 0 ? undefined : Decimal.__wrap(ret);
    }
    /**
     * Returns the value of this `Decimal` converted to a primitive number.
     *
     * # Note
     * At the time of writing, the conversion from `Decimal` to `f64` cannot fail. To guard
     * against future implementation changes, `f64::NAN` is returned as a fallback value.
     * @returns {number}
     */
    toNumber() {
        const ret = wasm.decimal_toNumber(this.__wbg_ptr);
        return ret;
    }
    /**
     * Returns the string representation of this `Decimal`.
     *
     * This intentionally overrides the default JS `toString()` so that string coercion
     * (e.g. template literals, `console.log`) produces the decimal representation.
     * @returns {string}
     */
    toString() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.decimal_toString(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export3(deferred1_0, deferred1_1, 1);
        }
    }
}
if (Symbol.dispose) Decimal.prototype[Symbol.dispose] = Decimal.prototype.free;

/**
 * Global context struct for KaTeX
 *
 * The KatexContext serves as the central registry for all KaTeX functionality,
 * containing mappings for functions, HTML/MathML builders, symbols, and
 * environments. This design enables runtime extensibility of LaTeX commands
 * without recompilation.
 */
export class KatexContext {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        KatexContextFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_katexcontext_free(ptr, 0);
    }
}
if (Symbol.dispose) KatexContext.prototype[Symbol.dispose] = KatexContext.prototype.free;

/**
 * Main error type thrown by KaTeX functions when something has gone wrong.
 * This is used to distinguish internal errors from errors in the expression
 * that the user provided.
 */
export class ParseError {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ParseErrorFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_parseerror_free(ptr, 0);
    }
}
if (Symbol.dispose) ParseError.prototype[Symbol.dispose] = ParseError.prototype.free;

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
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SettingsFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_settings_free(ptr, 0);
    }
    /**
     * Whether `\color` commands affect surrounding text color.
     *
     * When `true`, color commands modify text color. When `false`,
     * they only affect mathematical content.
     * @returns {boolean}
     */
    get color_is_text_color() {
        const ret = wasm.__wbg_get_settings_color_is_text_color(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Color value for mathematical content.
     *
     * CSS color value used for rendering mathematical expressions.
     * @returns {string | undefined}
     */
    get color() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_settings_color(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_export3(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Whether mathematical expressions are rendered in display (block) mode.
     *
     * When `true`, equations are centered and displayed on separate lines
     * with larger fonts. When `false`, equations are rendered inline.
     * @returns {boolean}
     */
    get display_mode() {
        const ret = wasm.__wbg_get_settings_display_mode(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * CSS color value used for rendering error messages.
     *
     * Applied to error text when `throw_on_error` is `false`.
     * @returns {string}
     */
    get error_color() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_settings_error_color(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export3(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Whether equations are flushed to the left margin.
     *
     * When `true`, equations are left-aligned instead of centered.
     * @returns {boolean}
     */
    get fleqn() {
        const ret = wasm.__wbg_get_settings_fleqn(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Whether settings persist globally across render calls.
     *
     * When `true`, settings remain active for subsequent expressions.
     * @returns {boolean}
     */
    get global_group() {
        const ret = wasm.__wbg_get_settings_global_group(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Whether equation numbers are placed on the left side.
     *
     * Controls the positioning of equation numbers in numbered environments.
     * @returns {boolean}
     */
    get leqno() {
        const ret = wasm.__wbg_get_settings_leqno(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Maximum limit for macro expansion iterations.
     *
     * Prevents infinite loops in macro expansion.
     * @returns {number}
     */
    get max_expand() {
        const ret = wasm.__wbg_get_settings_max_expand(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Maximum allowed size for rendered expressions.
     *
     * Prevents excessive memory usage from very large expressions. In points.
     * @returns {number}
     */
    get max_size() {
        const ret = wasm.__wbg_get_settings_max_size(this.__wbg_ptr);
        return ret;
    }
    /**
     * Minimum thickness for rendered rules (lines).
     *
     * Prevents lines from becoming too thin to be visible. In points.
     * @returns {number}
     */
    get min_rule_thickness() {
        const ret = wasm.__wbg_get_settings_min_rule_thickness(this.__wbg_ptr);
        return ret;
    }
    /**
     * Size multiplier for scaling rendered expressions.
     *
     * Controls the overall size scaling factor for mathematical expressions.
     * @returns {number}
     */
    get size_multiplier() {
        const ret = wasm.__wbg_get_settings_size_multiplier(this.__wbg_ptr);
        return ret;
    }
    /**
     * Whether parsing/rendering errors should throw exceptions.
     *
     * When `true`, errors cause panics. When `false`, errors are rendered
     * as colored text in the output.
     * @returns {boolean}
     */
    get throw_on_error() {
        const ret = wasm.__wbg_get_settings_throw_on_error(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Whether `\color` commands affect surrounding text color.
     *
     * When `true`, color commands modify text color. When `false`,
     * they only affect mathematical content.
     * @param {boolean} arg0
     */
    set color_is_text_color(arg0) {
        wasm.__wbg_set_settings_color_is_text_color(this.__wbg_ptr, arg0);
    }
    /**
     * Color value for mathematical content.
     *
     * CSS color value used for rendering mathematical expressions.
     * @param {string | null} [arg0]
     */
    set color(arg0) {
        var ptr0 = isLikeNone(arg0) ? 0 : passStringToWasm0(arg0, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        var len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_settings_color(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * Whether mathematical expressions are rendered in display (block) mode.
     *
     * When `true`, equations are centered and displayed on separate lines
     * with larger fonts. When `false`, equations are rendered inline.
     * @param {boolean} arg0
     */
    set display_mode(arg0) {
        wasm.__wbg_set_settings_display_mode(this.__wbg_ptr, arg0);
    }
    /**
     * CSS color value used for rendering error messages.
     *
     * Applied to error text when `throw_on_error` is `false`.
     * @param {string} arg0
     */
    set error_color(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_settings_error_color(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * Whether equations are flushed to the left margin.
     *
     * When `true`, equations are left-aligned instead of centered.
     * @param {boolean} arg0
     */
    set fleqn(arg0) {
        wasm.__wbg_set_settings_fleqn(this.__wbg_ptr, arg0);
    }
    /**
     * Whether settings persist globally across render calls.
     *
     * When `true`, settings remain active for subsequent expressions.
     * @param {boolean} arg0
     */
    set global_group(arg0) {
        wasm.__wbg_set_settings_global_group(this.__wbg_ptr, arg0);
    }
    /**
     * Whether equation numbers are placed on the left side.
     *
     * Controls the positioning of equation numbers in numbered environments.
     * @param {boolean} arg0
     */
    set leqno(arg0) {
        wasm.__wbg_set_settings_leqno(this.__wbg_ptr, arg0);
    }
    /**
     * Maximum limit for macro expansion iterations.
     *
     * Prevents infinite loops in macro expansion.
     * @param {number} arg0
     */
    set max_expand(arg0) {
        wasm.__wbg_set_settings_max_expand(this.__wbg_ptr, arg0);
    }
    /**
     * Maximum allowed size for rendered expressions.
     *
     * Prevents excessive memory usage from very large expressions. In points.
     * @param {number} arg0
     */
    set max_size(arg0) {
        wasm.__wbg_set_settings_max_size(this.__wbg_ptr, arg0);
    }
    /**
     * Minimum thickness for rendered rules (lines).
     *
     * Prevents lines from becoming too thin to be visible. In points.
     * @param {number} arg0
     */
    set min_rule_thickness(arg0) {
        wasm.__wbg_set_settings_min_rule_thickness(this.__wbg_ptr, arg0);
    }
    /**
     * Size multiplier for scaling rendered expressions.
     *
     * Controls the overall size scaling factor for mathematical expressions.
     * @param {number} arg0
     */
    set size_multiplier(arg0) {
        wasm.__wbg_set_settings_size_multiplier(this.__wbg_ptr, arg0);
    }
    /**
     * Whether parsing/rendering errors should throw exceptions.
     *
     * When `true`, errors cause panics. When `false`, errors are rendered
     * as colored text in the output.
     * @param {boolean} arg0
     */
    set throw_on_error(arg0) {
        wasm.__wbg_set_settings_throw_on_error(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) Settings.prototype[Symbol.dispose] = Settings.prototype.free;

/**
 * @param {string} query
 * @param {number} page
 * @param {number} per_page
 * @returns {any}
 */
export function search_conundrum_emojis(query, page, per_page) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(query, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len0 = WASM_VECTOR_LEN;
        wasm.search_conundrum_emojis(retptr, ptr0, len0, page, per_page);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        if (r2) {
            throw takeObject(r1);
        }
        return takeObject(r0);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * @param {string} query
 * @param {string} container_id
 * @param {number} page
 * @param {number} per_page
 * @returns {any}
 */
export function search_conundrum_emojis_in_docs_container(query, container_id, page, per_page) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(query, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(container_id, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len1 = WASM_VECTOR_LEN;
        wasm.search_conundrum_emojis_in_docs_container(retptr, ptr0, len0, ptr1, len1, page, per_page);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        if (r2) {
            throw takeObject(r1);
        }
        return takeObject(r0);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg___wbindgen_throw_6ddd609b62940d55: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbg_new_a70fbab9066b301f: function() {
            const ret = new Array();
            return addHeapObject(ret);
        },
        __wbg_new_ab79df5bd7c26067: function() {
            const ret = new Object();
            return addHeapObject(ret);
        },
        __wbg_set_282384002438957f: function(arg0, arg1, arg2) {
            getObject(arg0)[arg1 >>> 0] = takeObject(arg2);
        },
        __wbg_set_6be42768c690e380: function(arg0, arg1, arg2) {
            getObject(arg0)[takeObject(arg1)] = takeObject(arg2);
        },
        __wbindgen_cast_0000000000000001: function(arg0) {
            // Cast intrinsic for `F64 -> Externref`.
            const ret = arg0;
            return addHeapObject(ret);
        },
        __wbindgen_cast_0000000000000002: function(arg0, arg1) {
            // Cast intrinsic for `Ref(String) -> Externref`.
            const ret = getStringFromWasm0(arg0, arg1);
            return addHeapObject(ret);
        },
        __wbindgen_object_clone_ref: function(arg0) {
            const ret = getObject(arg0);
            return addHeapObject(ret);
        },
        __wbindgen_object_drop_ref: function(arg0) {
            takeObject(arg0);
        },
    };
    return {
        __proto__: null,
        "./conundrum_wasm_bg.js": import0,
    };
}

const DecimalFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_decimal_free(ptr >>> 0, 1));
const KatexContextFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_katexcontext_free(ptr >>> 0, 1));
const ParseErrorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_parseerror_free(ptr >>> 0, 1));
const SettingsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_settings_free(ptr >>> 0, 1));

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function dropObject(idx) {
    if (idx < 1028) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getObject(idx) { return heap[idx]; }

let heap = new Array(1024).fill(undefined);
heap.push(undefined, null, true, false);

let heap_next = heap.length;

function isLikeNone(x) {
    return x === undefined || x === null;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;

let wasmModule, wasm;
function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    wasmModule = module;
    cachedDataViewMemory0 = null;
    cachedUint8ArrayMemory0 = null;
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && expectedResponseType(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else { throw e; }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);
    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };
        } else {
            return instance;
        }
    }

    function expectedResponseType(type) {
        switch (type) {
            case 'basic': case 'cors': case 'default': return true;
        }
        return false;
    }
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (module_or_path === undefined) {
        module_or_path = new URL('conundrum_wasm_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
