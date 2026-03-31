//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 3/31/26.
//

import ConundrumSwift
import Foundation

public struct ConundrumTextUtils {
  public init() {
  }
  public static func getTitleFromConundrum(content: String) async -> TitleGroup? {
    do {
      let res = try await getTitle(
        content: content, modifiers: [.preferInlineMarkdownSyntax, .preferMarkdownSyntax])
      return res
    } catch {
      print("Error retrieving note title: \(error.localizedDescription)")
    }
    return nil
  }
}
