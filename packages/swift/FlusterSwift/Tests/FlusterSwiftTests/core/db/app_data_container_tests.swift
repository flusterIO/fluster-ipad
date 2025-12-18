import Testing
//@testable import SwiftData
//@testable import SwiftUI
@testable import FlusterRust
@testable import FlusterSwift

@Test("Decodes initial json data")
func testInitialJsonDecoding() async throws {
    let noteData = InitialNotesDataJsonDecoder.decode(
        from: INITIAL_NOTES_DATA_PATH
    )
    if let nd = noteData {
        #expect(!nd.isEmpty, "Note data is not empty")
    } else {
        Issue.record("Failed to parse initial note data.")
    }
}
