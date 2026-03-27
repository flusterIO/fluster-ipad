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
    You are an assistant for a note taking application for STEM students and professionals. Summarize the user's note in no more than 250 words, much like an abstract for a peer reviewed paper.
    """
    if !details.preferred_name.isEmpty {
      details.toUserDescription()
    }
    """
    They're note content is:
    """
  })
  print("Session: \(session.transcript)")
  let res = try await session.respond(to: focusedNote.markdown.body)
  print("Summary: \(res.content)")
  return AiPhase2Response(
    success: false, replaceWith: nil, res: res.content, userMessage: nil, id: nil, model: "")
}
