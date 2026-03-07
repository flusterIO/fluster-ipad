import { SizableOption, WebviewContainerState } from "@/code_gen/typeshare/fluster_core_utilities";

export const initialWebviewContainerState: Omit<WebviewContainerState, "environment"> = {
    size: SizableOption.None,
    wasm_loaded: false,
    dark_mode: true
}
