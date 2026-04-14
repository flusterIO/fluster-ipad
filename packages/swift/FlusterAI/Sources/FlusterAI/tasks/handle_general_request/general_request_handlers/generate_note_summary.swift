//
//  File.swift
//  FlusterAI
//
//  Created by Andrew on 3/23/26.
//

import FlusterData
import Foundation
import FoundationModels

public func getNoteSummaryLanguageModelSession(_ details: AIUserDetails) -> LanguageModelSession {
  let session = LanguageModelSession(instructions: {
    """
    You are a brilliant assistant for a top tier academic's note taking application. Write an abstract for this note as if it were a peer reviewed paper, in no more than 150 words.

    Your summary should be short, concise and informative, relaying to users at first glance exactly what is presented in this note.

    An example summary:

    "Dr. Philips reviews his findings regarding the recent cooperative effort with Dr. Stevens. The efforts seem to be showing promise as demonstrated by a MAPE of just 0.3%. The note goes on to show this agreement through a series of plots, and is finalized by a short plan to follow through with Dr. Stevens."


    """
    if details.preferred_name?.isEmpty == false {
      details.toUserDescription()
    }
    """
    Their note content is:
    """
  })
  return session
}

public func generateNoteSummary(focusedNote: NoteModel, details: AIUserDetails)
  async throws -> AiPhase2Response
{
  let session = getNoteSummaryLanguageModelSession(details)
  let content = try await focusedNote.markdown.body.conundrumToAIInput(noteId: focusedNote.id)
  print("AI Input: \(content)")
  let res = try await session.respond(to: content)
  return AiPhase2Response(
    success: true, replaceWith: nil, res: content, userMessage: nil, id: nil,
    model: .localFoundationModels)
}
