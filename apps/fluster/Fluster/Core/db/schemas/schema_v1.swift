//
//  schema_v1.swift
//  Fluster
//
//  Created by Andrew on 11/27/25.
//

import Foundation
import PencilKit
import SwiftData
import SwiftyBibtex

enum AppSchemaV1: VersionedSchema {
    static var models: [any PersistentModel.Type] {
        [NoteModel.self, BibEntryModel.self]  // Leaving off other models as they should be able to be inferred and this will simplify migration scripts I hope.
    }
    static var versionIdentifier: Schema.Version = .init(1, 0, 0)
}

extension AppSchemaV1 {
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

        static func fromNoteBody(noteBody: String) -> NoteModel {
            let note = NoteModel(
                drawing: PKDrawing().dataRepresentation(),
                markdown: MarkdownNote(body: noteBody, summary: nil)
            )
            return note
        }
    }
    @Model
    class BibEntryModel: Identifiable {
        @Attribute(.unique) var id: String
        @Attribute(.unique) var citationKey: String?
        /// The bibtex string representing the item's data.
        var data: String
        var ctime: Date
        var utime: Date
        var notes = [NoteModel]()
        init(
            id: String? = nil,
            data: String,
            ctime: Date = .now,
            utime: Date = .now,
            notes: [NoteModel] = []
        ) {
            self.id = id ?? UUID().uuidString
            self.data = data
            self.ctime = ctime
            self.utime = utime
            self.notes = notes
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

        func getTitle() -> String {
            return self.parse()?.publications[0].fields["title"] ?? "--"
        }
    }

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

    @Model
    class SubjectModel {
        @Attribute(.unique) var value: String
        var ctime: Date
        var utime: Date
        @Relationship(inverse: \NoteModel.subject) var notes: [NoteModel] = []
        init(
            value: String,
            ctime: Date = .now,
            utime: Date = .now,
            notes: [NoteModel] = []
        ) {
            self.value = value
            self.ctime = ctime
            self.utime = utime
            self.notes = notes
        }
    }

    @Model
    class TopicModel {
        @Attribute(.unique) var value: String
        var ctime: Date
        var utime: Date
        @Relationship(inverse: \NoteModel.topic) var notes: [NoteModel] = []
        init(
            value: String,
            ctime: Date = .now,
            utime: Date = .now,
            notes: [NoteModel] = []
        ) {
            self.value = value
            self.ctime = ctime
            self.utime = utime
            self.notes = notes
        }
    }
}
