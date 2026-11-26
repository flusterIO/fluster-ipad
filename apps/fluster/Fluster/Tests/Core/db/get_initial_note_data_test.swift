//
//  get_initial_note_data_test.swift
//  Fluster
//
//  Created by Andrew on 12/18/25.
//

// `@testable` needs ENABLE_TESTABILITY, which is off in Release (archive) builds.
#if DEBUG
import Testing

@testable import FlusterSwift
@testable import FlusterData
#endif

//@Test("Decodes initial json data")
//func testInitialJsonDecoding() async throws {
//  let noteData = InitialNotesDataJsonDecoder.decode(
//    from: "initial_note_docs/initial_notes_parsed_data"
//  )
//  if let nd = noteData {
//    #expect(!nd.isEmpty, "Note data is not empty")
//  } else {
//    Issue.record("Failed to parse initial note data.")
//  }
//}
