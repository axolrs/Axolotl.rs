import XCTest
import SwiftTreeSitter
import TreeSitterAxolotlgrammer

final class TreeSitterAxolotlgrammerTests: XCTestCase {
    func testCanLoadGrammar() throws {
        let parser = Parser()
        let language = Language(language: tree_sitter_axolotlgrammer())
        XCTAssertNoThrow(try parser.setLanguage(language),
                         "Error loading Axolotl Grammer grammar")
    }
}
