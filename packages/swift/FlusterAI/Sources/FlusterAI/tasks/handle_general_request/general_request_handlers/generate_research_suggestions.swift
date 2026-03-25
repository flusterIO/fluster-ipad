//
//  File.swift
//  FlusterAI
//
//  Created by Andrew on 3/23/26.
//


import Foundation
import FoundationModels
import FlusterData

public func generateResearchSuggestions(req: GeneralAiRequestPhase2Event, focusedNote: NoteModel) async throws -> AiPhase2Response {
    let session = LanguageModelSession(instructions: {
        """
        You are an assistant for a note taking application for STEM students and professionals. Help this researcher further their knowledge by providing a list of suggestions for areas of further research related to the note provided.
        """
    })
    let res = try await session.respond(to: focusedNote.markdown.body)
    print("Res: \(res)")
    return AiPhase2Response(success: false, replaceWith: nil, userMessage: nil, id: nil)
}
