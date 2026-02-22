//
//  Fluster_DesktopTests.swift
//  Fluster DesktopTests
//
//  Created by Andrew on 1/14/26.
//

import Testing
@testable import Fluster
@testable import FlusterBibliography
@testable import FlusterData

struct Fluster_DesktopTests {

    @Test func readsEmbedddCslFiles() async throws {
        EmbeddedCslFileSwift.allCases.forEach{ csl in
            let f = csl.toFlusterBibliographyCslFile()
            assert(!f.isEmpty, "\(csl.rawValue) is not empty.")
        }
    }
    
    @Test func getsEmbededCslLocaleFile() async throws {
        let res = getCslLocaleFileContent()
        assert(!res.isEmpty, "Csl Locale file content is not empty.")
    }

}
