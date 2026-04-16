//
//  File.swift
//  FlusterAI
//
//  Created by Andrew on 3/27/26.
//

import ConundrumSwift
import Foundation

extension String {
  public func conundrumToAIInput(noteId: String?, uiParams: UiParams) async throws -> String {
    let res = try await ConundrumSwift.runConundrum(
      options: ParseConundrumOptions(
        noteId: noteId, content: self, modifiers: [.preferMarkdownSyntax, .forAiInput],
        hideComponents: [], uiParams: uiParams))
    return res.content
  }
  public func conundrumToSwiftCompatibleMarkdown(noteId: String?, inline: Bool, uiParams: UiParams)
    async throws -> String
  {
    let res = try await ConundrumSwift.runConundrum(
      options: ParseConundrumOptions(
        noteId: noteId, content: self, modifiers: [.preferMarkdownSyntax, .forAiInput],
        hideComponents: [], uiParams: uiParams))
    return res.content
  }
}
