//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/25/25.
//

import Testing

@testable import FlusterSwift

@MainActor
@Test("Parses Mdx successfully.")
func testMdxParsingResults() async throws {
  let nd = await getTestNote()
  print("Citations Length: \(nd.citations.count)")
  #expect(!nd.citations.isEmpty, "Note parses citations successfully.")
  #expect(!nd.dictionaryEntries.isEmpty, "Note parses dictionary entries successfully.")
}
