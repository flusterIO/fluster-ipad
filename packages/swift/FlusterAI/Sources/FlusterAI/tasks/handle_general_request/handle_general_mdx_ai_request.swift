//
//  File.swift
//  FlusterAI
//
//  Created by Andrew on 3/23/26.
//

import FlusterData
import Foundation
import FoundationModels

public func handleGeneralMdxAiRequest(request: GeneralAiRequestPhase2Event, focusedNote: NoteModel?)
  -> AiPhase2Response
{
  let session = LanguageModelSession(instructions: {
    """
    You are an assistant for a note taking application built for STEM students and professionals.
    Throughout this application there is an Enum of actions that the user can suggest be taken on their behalf.
    Evalualate their input, and select the action that they are most likely asking for. If they don't appear to be asking for any valid action, return .SeeminglyUnrelated
    """
  })
  // Required to avoid a data race apparently... The rust side of me wants to cry.
  let user_request = request.user_request

  Task(priority: .high) {
    do {
      print("Request: \(user_request)")
      let res = try await session.respond(to: user_request, generating: AiNoteInteractionType.self)
      print("Intermediary: \(res)")
    } catch {
      print("Error: \(error.localizedDescription)")
    }
  }
    return AiPhase2Response(success: false, replaceWith: nil, userMessage: nil, id: nil, model: "")
}
