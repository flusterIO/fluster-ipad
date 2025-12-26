//
//  File 2.swift
//  FlusterSwift
//
//  Created by Andrew on 12/4/25.
//

import FlusterRust
import Foundation

struct CodableCitationResult: Codable {
  public let citation_key: String
  public let idx: UInt8

  static func fromRustResult(res: CitationResult) -> CodableCitationResult {
      CodableCitationResult(citation_key: res.citationKey, idx: res.idx)
  }
}

struct EditorWebviewData: Codable {
  let content: String
  /// citations is a single string of the concatenated bibtex entries. Make sure to remove duplicates and place them in the proper order before generating string.
  let citations: [CodableCitationResult]

  public static func fromRustResults(data: MdxParsingResult)
    -> EditorWebviewData
  {
    return EditorWebviewData(
      content: data.content,
      citations: data.citations.map({ cit in
        CodableCitationResult.fromRustResult(res: cit)
      })
    )
  }
}
