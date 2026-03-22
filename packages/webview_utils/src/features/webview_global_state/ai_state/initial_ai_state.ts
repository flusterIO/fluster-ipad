import { FoundationModelAccessStatus, type AiState } from "@/code_gen/typeshare/fluster_core_utilities";

export const initialAiState: AiState = {
    foundation_model_access: FoundationModelAccessStatus.UnknownStatus,
    ai_thinking: false
}
