import { compile_cdrm } from "../../../pkg/conundrum_wasm.js";

export const compileConundrum = async (
    content: string,
    /**
     * This is the `UIPaarams` object exported from the `@conundrum/ts` package, but do to some circular dependeny issues, and the fact that I'm hours away from releasing this after 5 years without a paycheck, the typesafety will have to wat.
     * Returns `Promise<MdxParsingResult>`
     */
    uiParams: object,
): Promise<object> => {
    const r = await compile_cdrm(content, uiParams, true) as object;
    return r
};
