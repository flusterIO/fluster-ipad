//
//  File.swift
//  FlusterAI
//
//  Created by Andrew on 3/23/26.
//

import Foundation
import FoundationModels
import FlusterData

public func generateNoteStudyGuide(req: GeneralAiRequestPhase2Event, focusedNote: NoteModel) async throws -> AiPhase2Response {
    let session = LanguageModelSession(instructions: {
        """
        You are an assistant for a note taking application for STEM students and professionals. Help the user study efficiently by providing a thorough study guide for the following note.
        """
    })
    
    let res = try await session.respond(to: focusedNote.markdown.body)
    return AiPhase2Response(success: false, replaceWith: nil, userMessage: nil, id: nil)
}
