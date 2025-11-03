//
//  bib_entry_model.swift
//  Fluster
//
//  Created by Andrew on 11/2/25.
//

import Foundation
import SwiftData
import SwiftyBibtex

@Model
class BibEntryModel {
    @Attribute(.unique) var id: UUID?
    @Attribute(.unique) var citationKey: String?
    /// The bibtex string representing the item's data.
    var data: String
    var ctime: Date
    init(id: UUID? = nil, data: String, ctime: Date = .now) {
        self.id = id
        self.data = data
        self.ctime = ctime
        self.citationKey = self.getCitationKey()
    }
    func parse() -> SwiftyBibtex.BibtexResult? {
        let result = try? SwiftyBibtex.parse(self.data)
        if result != nil {
            for warning in result!.warnings {
                print(warning.message)
            }
            if result!.publications.count == 1 {
                return result
            }
        }
        return nil
    }

    func getCitationKey() -> String? {
        let result = try? SwiftyBibtex.parse(self.data)
        if result != nil {
            for warning in result!.warnings {
                print(warning.message)
            }
            if result!.publications.count == 1 {
                return result!.publications[0].citationKey
            }
        }
        return nil
    }
}
