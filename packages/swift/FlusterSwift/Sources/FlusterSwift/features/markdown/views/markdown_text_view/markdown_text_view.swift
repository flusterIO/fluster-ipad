//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 3/9/26.
//

import SwiftUI

public struct MarkdownTextView: View {
  let content: String
  let syntax: AttributedString.MarkdownParsingOptions.InterpretedSyntax
  private var attributedContent: AttributedString {
    do {
      return try AttributedString(
        markdown: content,
        options: .init(interpretedSyntax: syntax)
      )
    } catch {
      return AttributedString(content)
    }
  }
  public init(
    _ content: String, _ syntax: AttributedString.MarkdownParsingOptions.InterpretedSyntax
  ) {
    self.content = content
    self.syntax = syntax
  }
  public var body: some View {
    Text(attributedContent)
  }
}

#Preview {
  MarkdownTextView("My _markdown_ text.", .inlineOnly)
}
