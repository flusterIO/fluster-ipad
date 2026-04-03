import { WebviewImplementation, type WebviewContainerState, FlusterTheme, WebviewFontSize, WebviewEnvironment } from "@/code_gen/typeshare/fluster_core_utilities";
import { type WithNullableOptionals } from "../../../../core/utils/types/utility_types";
import { SizableOption } from "@/code_gen/typeshare/conundrum";

export const initialWebviewContainerState: WithNullableOptionals<WebviewContainerState> = {
    size: SizableOption.None,
    wasm_loaded: false,
    dark_mode: true,
    implementation: WebviewImplementation.MdxEditor,
    fluster_theme: FlusterTheme.Fluster,
    font_size: WebviewFontSize.Base,
    environment: WebviewEnvironment.AwaitingData
}
