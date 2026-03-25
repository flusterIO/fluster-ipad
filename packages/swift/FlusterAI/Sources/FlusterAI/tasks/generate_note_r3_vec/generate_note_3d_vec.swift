//
//  File.swift
//  FlusterAI
//
//  Created by Andrew on 3/22/26.
//

import Foundation
import FlusterData
import FoundationModels


@Generable(description: "A vector in 3 dimensional space showing the note's relationship to the x-axis label, y-axis label and x-axis label properties.")
public struct GeneratedR3Vec {
    @Guide(description: "The value between 0 and 1 representing the note's relationship to the subject indicated by the x-axis label.")
    public var xAxisValue: Float
    @Guide(description: "The value between 0 and 1 representing the note's relationship to the subject indicated by the y-axis label.")
    public var yAxisValue: Float
    @Guide(description: "The value between 0 and 1 representing the note's relationship to the subject indicated by the z-axis label.")
    public var zAxisValue: Float
    public init(xAxisValue: Float, yAxisValue: Float, zAxisValue: Float) {
        self.xAxisValue = xAxisValue
        self.yAxisValue = yAxisValue
        self.zAxisValue = zAxisValue
    }
}

public func generate_note_R3_vec(note: NoteModel, xAxisLabel: String, yAxisLabel: String, zAxisLabel: String) async throws -> GeneratedR3Vec {
    let session = LanguageModelSession {
        """
        You are an assistant for a note taking application built for STEM students and professionals.
        
        For the given note content, rate the note's relationship to the following 3 subjects.
        
        The first subject indicates the x-axis, the second subject indicates the y-axis, and the third subject indicates the z-axis.
        """
        
        """
        The labels for the 3 axis' are:
        
        x-axis label: "\(xAxisLabel)"
        y-axis label: "\(yAxisLabel)"
        z-axis label: "\(zAxisLabel)"
        """
    }
    let res = try await session.respond(to: note.markdown.body, generating: GeneratedR3Vec.self)
    return res.content
}
