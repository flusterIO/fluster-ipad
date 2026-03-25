//
//  File.swift
//  FlusterAI
//
//  Created by Andrew on 3/23/26.
//

import FlusterData
import Foundation
import FoundationModels

@Generable(description: "A general action that the user can request be taken on their behalf within our note taking application.")
public enum AiNoteInteractionType: String {
  case GenerateSummary = "The user is requesting that a summary of their note be created."
  case GenerateNoteStudyGuide = "The user is asking for a study guide related to their note."
  case SuggestFurtherResearch = "The user is asking for further research suggestions related to their note."
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
          success: false, replaceWith: nil,
          userMessage:
            "I'm sorry. Fluster couldn't figure out which supported task you were trying to run. See the `AI??` docs for a list of currently supported actions.",
          id: nil,
          model: ""
        )
    }
  }
}
