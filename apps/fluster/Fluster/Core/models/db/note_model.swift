//
//  note_model.swift
//  Fluster
//
//  Created by Andrew on 11/2/25.
//

import Foundation
import PencilKit
import SwiftData

func loadDrawing(note: NoteModel) -> PKDrawing {
    do {
        // Recreate the PKDrawing from the Data
        return try PKDrawing(data: note.drawing)
    } catch {
        print("Error loading PKDrawing: \(error)")
        return PKDrawing()
    }
}

@Model
class NoteModel {
    var id: String
    @Attribute(.externalStorage) var drawing: Data
    var markdown: MarkdownNote = MarkdownNote(body: "", summary: nil)
    var ctime: Date
    var utime: Date
    var last_read: Date

    var subject: SubjectModel? = nil
    var topic: TopicModel? = nil
    var tags = [TagModel]()
    @Relationship(inverse: \BibEntryModel.notes)
    var citations = [BibEntryModel]()

    // drawing.toDataRepresentation() to conform to Data type.
    init(
        id: String? = nil,
        drawing: Data,
        markdown: MarkdownNote = MarkdownNote(body: "", summary: nil),
        ctime: Date = .now,
        utime: Date = .now,
        last_read: Date = .now,
        subject: SubjectModel? = nil,
        topic: TopicModel? = nil,
        tags: [TagModel] = [TagModel](),
        citations: [BibEntryModel] = [BibEntryModel]()
    ) {
        self.id = id ?? UUID.init().uuidString
        self.drawing = drawing
        self.markdown = markdown
        self.ctime = ctime
        self.utime = utime
        self.last_read = last_read
        self.subject = subject
        self.topic = topic
        self.tags = tags
        self.citations = citations
    }

    func containsCitation(citation: BibEntryModel) -> Bool {
        return self.citations.contains(where: { $0.id == citation.id })
    }

    static func count(modelContext: ModelContext) -> Int {
        let descriptor = FetchDescriptor<BibEntryModel>()
        return (try? modelContext.fetchCount(descriptor)) ?? 0
    }

    static func isTitleMatch(noteBody: String, query: String) -> Bool {
        let title = getTitle(body: noteBody)
        return title == nil
            ? false : title!.localizedCaseInsensitiveContains(query)
    }
}
