//
//  topic_model.swift
//  Fluster
//
//  Created by Andrew on 11/2/25.
//

import Foundation
import SwiftData

@Model
class TopicModel {
    @Attribute(.unique) var value: String
    var ctime: Date
    var utime: Date
    @Relationship(inverse: \NoteModel.subject) var notes: [NoteModel] = []
    init(value: String, ctime: Date = .now, utime: Date = .now, notes: [NoteModel] = []) {
        self.value = value
        self.ctime = ctime
        self.utime = utime
        self.notes = notes
    }
}
