//
//  File.swift
//  FlusterAI
//
//  Created by Andrew on 3/27/26.
//

import Foundation
import ConundrumSwift


public extension String {
    func conundrumToAIInput(noteId: String?) async throws -> String {
       let res = try await ConundrumSwift.runConundrum(options: ParseMdxOptions(noteId: noteId, content: self, modifiers: [.preferMarkdownSyntax, .forAiInput]))
        return res.content
    }
    func conundrumToSwiftCompatibleMarkdown(noteId: String?, inline: Bool) async throws -> String {
       let res = try await ConundrumSwift.runConundrum(options: ParseMdxOptions(noteId: noteId, content: self, modifiers: [.preferMarkdownSyntax, .forAiInput]))
        return res.content
    }
}
