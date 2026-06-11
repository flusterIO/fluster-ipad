import { compile_cdrm } from "../../../pkg/conundrum_wasm.js";

/* eslint-disable-next-line  --  */
export const compileConundrum = <UIParams, CdrmModifer, CdrmParsingResult>(
    content: string,
    uiParams: UIParams,
    modifiers: CdrmModifer[],
    trusted: boolean,
): CdrmParsingResult => {
    return compile_cdrm(
        content,
        uiParams,
        modifiers,
        trusted,
    ) as CdrmParsingResult;
};
