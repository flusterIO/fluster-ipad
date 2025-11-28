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

public enum AppSchemaV1: VersionedSchema {
    public static var models: [any PersistentModel.Type] {
        [NoteModel.self, BibEntryModel.self]  // Leaving off other models as they should be able to be inferred and this will simplify migration scripts I hope.
    }
    public static let versionIdentifier: Schema.Version = .init(1, 0, 0)
}

extension AppSchemaV1 {
    @Model
    public class NoteModel {
        public var id: String
        @Attribute(.externalStorage) public var drawing: Data
        public var markdown: MarkdownNote = MarkdownNote(body: "", summary: nil)
        public var ctime: Date
        public var utime: Date
        public var last_read: Date

        public var subject: SubjectModel? = nil
        public var topic: TopicModel? = nil
        public var tags = [TagModel]()
        @Relationship(inverse: \BibEntryModel.notes)
        public var citations = [BibEntryModel]()

        // drawing.toDataRepresentation() to conform to Data type.
        public init(
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

        public func containsCitation(citation: BibEntryModel) -> Bool {
            return self.citations.contains(where: { $0.id == citation.id })
        }

        public static func count(modelContext: ModelContext) -> Int {
            let descriptor = FetchDescriptor<BibEntryModel>()
            return (try? modelContext.fetchCount(descriptor)) ?? 0
        }

        public static func isTitleMatch(noteBody: String, query: String) -> Bool
        {
            let title = MdxTextUtils.getTitle(body: noteBody)
            return title == nil
                ? false : title!.localizedCaseInsensitiveContains(query)
        }

        public static func fromNoteBody(noteBody: String) -> NoteModel {
            let note = NoteModel(
                drawing: PKDrawing().dataRepresentation(),
                markdown: MarkdownNote(body: noteBody, summary: nil)
            )
            return note
        }
    }
    @Model
    public class BibEntryModel: Identifiable {
        @Attribute(.unique) public var id: String
        @Attribute(.unique) public var citationKey: String?
        /// The bibtex string representing the item's data.
        public var data: String
        public var ctime: Date
        public var utime: Date
        public var notes = [NoteModel]()
        public init(
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
        public func parse() -> SwiftyBibtex.BibtexResult? {
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

        public func getCitationKey() -> String? {
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

        public func getTitle() -> String {
            return self.parse()?.publications[0].fields["title"] ?? "--"
        }
    }

    @Model
    public class TagModel {
        @Attribute(.unique) public var value: String
        public var ctime: Date
        @Relationship(inverse: \NoteModel.tags) public var notes: [NoteModel] =
            []
        public init(value: String, ctime: Date = .now) {
            self.value = value
            self.ctime = ctime
        }
    }

    @Model
    public class SubjectModel {
        @Attribute(.unique) public var value: String
        public var ctime: Date
        public var utime: Date
        @Relationship(inverse: \NoteModel.subject) public var notes:
            [NoteModel] = []
        public init(
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
    public class TopicModel {
        @Attribute(.unique) public var value: String
        public var ctime: Date
        public var utime: Date
        @Relationship(inverse: \NoteModel.topic) public var notes: [NoteModel] =
            []
        public init(
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
    public class MarkdownNote {
        public var _body: String
        public var title: String?
        public var summary: String?

        public init(body: String, summary: String?) {
            self._body = body
            self.summary = summary
            self.title = MdxTextUtils.getTitle(body: body)
        }

        public var body: String {
            get {
                return _body
            }
            set(newBody) {
                self._body = newBody
                self.title = MdxTextUtils.getTitle(body: newBody)
            }
        }

        /// Parses the structs body and returns the modified text in a string. This is where tags and other mdx content generated by non-traditional mdx syntax is inserted.
        public func parsedBodyText() -> String {
            let t = MdxText(body: self.body)
            t.parse()
            return t.body
        }
    }
}
