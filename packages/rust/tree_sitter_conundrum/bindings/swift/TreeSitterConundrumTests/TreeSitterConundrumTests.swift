import XCTest
import SwiftTreeSitter
import TreeSitterConundrum

final class TreeSitterConundrumTests: XCTestCase {
    func testCanLoadGrammar() throws {
        let parser = Parser()
        let language = Language(language: tree_sitter_conundrum())
        XCTAssertNoThrow(try parser.setLanguage(language),
                         "Error loading Conundrum grammar")
    }
}
