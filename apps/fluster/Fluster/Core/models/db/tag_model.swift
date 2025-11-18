//
//  tag_model.swift
//  Fluster
//
//  Created by Andrew on 11/2/25.
//

import Foundation
import SwiftData

@Model
class TagModel {
    @Attribute(.unique) var value: String
    var ctime: Date
    @Relationship(inverse: \NoteModel.tags) var notes: [NoteModel] = []
    init(value: String, ctime: Date = .now) {
        self.value = value
        self.ctime = ctime
    }
}
