//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/11/25.
//

import Foundation
import SwiftData
import FlusterData
import ConundrumSwift

@MainActor
public func getTestNote(modelContext: ModelContext, uiParams: UiParams) async -> NoteModel {
    do {
        let testNoteUrl = try URL("file:///Users/bigsexy/Desktop/notes/content/physics/ipad_app_notes/on_the_gravitational_nature_of_time.mdx", strategy: .url)
        let fileContents = try String(contentsOf: testNoteUrl, encoding: .utf8)
        let res = NoteModel(
          markdown: MarkdownNote(
            body: fileContents,
          )
        )
        if let parsingResults = await fileContents.preParseAsMdx(noteId: nil, uiParams: uiParams) {
            res.applyMdxParsingResults(results: parsingResults, modelContext: modelContext)
        }
        return res
    } catch {
        print("Error getting test note: \(error)")
        fatalError("Error getting test note: \(error)")
    }
}
