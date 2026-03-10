import { WebviewImplementation, SizableOption, type WebviewContainerState, FlusterTheme } from "@/code_gen/typeshare/fluster_core_utilities";

export const initialWebviewContainerState: Omit<WebviewContainerState, "environment"> = {
    size: SizableOption.None,
    wasm_loaded: false,
    dark_mode: true,
    implementation: WebviewImplementation.MdxEditor,
    fluster_theme: FlusterTheme.Fluster
}
