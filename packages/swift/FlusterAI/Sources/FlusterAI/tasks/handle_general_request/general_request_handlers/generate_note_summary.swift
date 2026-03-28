//
//  File.swift
//  FlusterAI
//
//  Created by Andrew on 3/23/26.
//

import FlusterData
import Foundation
import FoundationModels

public func generateNoteSummary(focusedNote: NoteModel, details: AIUserDetails)
  async throws -> AiPhase2Response
{
  let session = LanguageModelSession(instructions: {
    """
    You are an assistant for a note taking application for STEM students and professionals. Write an abstract for this note as if it were a peer reviewed paper, in no more than 250 words.
    """
    if !details.preferred_name.isEmpty {
      details.toUserDescription()
    }
    """
    They're note content is:
    """
  })
  let res = try await session.respond(to: focusedNote.markdown.body)
  let content = try await res.content.mdxToAIInput(noteId: focusedNote.id)
  print("AI Input: \(content)")
  return AiPhase2Response(
    success: true, replaceWith: nil, res: content, userMessage: nil, id: nil,
    model: .localFoundationModels)
}
