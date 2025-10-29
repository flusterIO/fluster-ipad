//
//  md_note.swift
//  Fluster
//
//  Created by Andrew on 10/29/25.
//

import Foundation
import SwiftData

@Model
class MarkdownNote {
    var body: String
    var label: String
    var id: UUID
    /// The last update time.
    var utime: Date
    /// The create time.
    var ctime: Date
    /// The date the note was last accessed.
    var last_read: Date
    
    init(body: String, label: String, id: UUID, utime: Date = .now, ctime: Date = .now, last_read: Date = .now) {
        self.body = body
        self.label = label
        self.id = id
        self.utime = utime
        self.ctime = ctime
        self.last_read = last_read
    }
}
