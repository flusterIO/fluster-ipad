//
//  File.swift
//  FlusterAI
//
//  Created by Andrew on 3/23/26.
//

import FlusterData
import Foundation
import FoundationModels

public func generateNoteSummary(req: GeneralAiRequestPhase2Event, focusedNote: NoteModel)
  async throws -> AiPhase2Response
{
  let session = LanguageModelSession(instructions: {
    """
    You are an assistant for a note taking application for STEM students and professionals. Summarize the following note in no more than 250 words, much like a short abstract for a peer reviewed paper.
    """
  })
  let res = try await session.respond(to: focusedNote.markdown.body)
  print("Summary: \(res)")
  return AiPhase2Response(success: false, replace_with: nil, user_message: nil)
}
