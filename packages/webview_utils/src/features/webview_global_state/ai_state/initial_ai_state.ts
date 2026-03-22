import { FoundationModalAccessStatus, type AiState } from "@/code_gen/typeshare/fluster_core_utilities";

export const initialAiState: AiState = {
    foundation_model_access: FoundationModalAccessStatus.NotInstalled,
    ai_thinking: false
}
