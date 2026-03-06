import { WebviewEnvironment } from "@/code_gen/typeshare/fluster_core_utilities";
import { createFlusterStore } from "./store";

export const initializeMdxEditorState = (_env: WebviewEnvironment) => {
    return createFlusterStore({
    })
}
