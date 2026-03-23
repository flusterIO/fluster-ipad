//
//  File.swift
//  FlusterAI
//
//  Created by Andrew on 3/23/26.
//

import FlusterData
import Foundation
import FoundationModels

@Generable(description: "This represents a general action that the user can request.")
public enum AiNoteInteractionType: String {
  case GenerateSummary = "The user is requesting a summary of their note."
  case GenerateNoteStudyGuide = "The user is requesting a study guide be generated."
  case SuggestFurtherResearch = "The user is asking for further research ideas."
  case SeeminglyUnrelated = "The user's request seems unrelated to any of these other actions."

  public func toResponse(req: GeneralAiRequestPhase2Event, focusedNote: NoteModel) async throws
    -> AiPhase2Response
  {
    switch self {
      case .GenerateNoteStudyGuide:
        return try await generateNoteStudyGuide(req: req, focusedNote: focusedNote)
      case .GenerateSummary:
        return try await generateNoteSummary(req: req, focusedNote: focusedNote)
      case .SuggestFurtherResearch:
        return try await generateResearchSuggestions(req: req, focusedNote: focusedNote)
      case .SeeminglyUnrelated:
        return AiPhase2Response(
          success: false, replace_with: nil,
          user_message:
            "I'm sorry. Fluster couldn't figure out which supported task you were trying to run. See the `AI??` docs for a list of currently supported actions."
        )
    }
  }
}
