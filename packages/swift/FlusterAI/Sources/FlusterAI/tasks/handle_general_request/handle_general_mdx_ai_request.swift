//
//  File.swift
//  FlusterAI
//
//  Created by Andrew on 3/23/26.
//

import Foundation
import FoundationModels
import FlusterData

public func handleGeneralMdxAiRequest(request: GeneralAiRequestPhase2Event, focusedNote: NoteModel?) -> AiPhase2Response {
    let session = LanguageModelSession(instructions: {
        """
        You are an assistant for a note taking application built for STEM students and professionals.
        Evaluate the user's input and determine the action that should be taken.
        """
    })
    // Required to avoid a data race apparently... The rust side of me wants to cry.
    let user_request = request.user_request
    
    Task(priority: .high) {
        do {
            let res = try await session.respond(to: user_request, generating: AiNoteInteractionType.self)
            print("Intermediary: \(res)")
        } catch {
            print("Error: \(error.localizedDescription)")
        }
    }
    return AiPhase2Response(success: false, replace_with: nil, user_message: nil)
}
