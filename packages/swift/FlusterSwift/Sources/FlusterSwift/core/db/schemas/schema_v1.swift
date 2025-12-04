//
//  schema_v1.swift
//  Fluster
//
//  Created by Andrew on 11/27/25.
//

import FlusterRust
import FlusterSwift
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
            let descriptor = FetchDescriptor<NoteModel>()
            return (try? modelContext.fetchCount(descriptor)) ?? 0
        }

        public static func isTitleMatch(noteBody: String, query: String) -> Bool {
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
        public func applyMdxParsingResults(
            results: MdxParsingResult,
        ) {
            self.markdown.preParsedBody = results.content
            self.tags = results.tags.map { tag in
                TagModel.fromRustTagResult(t: tag, existingTags: self.tags)
            }
        }
        public func setLastRead() {
            self.last_read = .now
            for tag in self.tags {
                tag.lastAccess = .now
            }
            for bibEntry in self.citations {
                bibEntry.lastAccess = .now
            }
            if let subject = self.subject {
                subject.lastAccess = .now
            }
            if let topic = self.topic {
                topic.lastAccess = .now
            }
        }
    }
    @Model
    public class BibEntryModel: Identifiable {
        @Attribute(.unique) public var id: String
        @Attribute(.unique) public var citationKey: String?
        public var htmlFormattedCitation: String?
        /// The bibtex string representing the item's data.
        public var data: String
        public var ctime: Date
        public var utime: Date
        /// The time a note with this bibliography entry was last accessed.
        public var lastAccess: Date
        public var notes = [NoteModel]()
        public init(
            id: String? = nil,
            data: String,
            ctime: Date = .now,
            utime: Date = .now,
            lastAccess: Date = .now,
            notes: [NoteModel] = []
        ) {
            self.id = id ?? UUID().uuidString
            self.data = data
            self.ctime = ctime
            self.utime = utime
            self.notes = notes
            self.lastAccess = lastAccess
            // TODO: Figure out how to get the html formatted citation in rust and use that to generate a formatted citation for each item that can then be passed to the webview for bibliography creation.
            self.htmlFormattedCitation = nil
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
    public class TaggableModel {
        public var ctime: Date
        public var utime: Date
        /// lastAccess is (will be) updated each time a note is accessed that contains a given taggable to help with search ranking.
        public var lastAccess: Date

        public init(ctime: Date, utime: Date, lastAccess: Date) {
            self.ctime = ctime
            self.utime = utime
            self.lastAccess = lastAccess
        }
    }
    @available(iOS 26, *)
    @Model
    public class TagModel: TaggableModel {
        @Attribute(.unique) public var value: String
        @Relationship(inverse: \NoteModel.tags) public var notes: [NoteModel] =
            []
        public init(
            value: String,
            ctime: Date = .now,
            utime: Date = .now,
            lastAccess: Date = .now,
            notes: [NoteModel] = []
        ) {
            //            super.init(ctime: ctime, utime: utime, lastAccess: lastAccess)
            self.value = value
            self.notes = notes
            super.init(ctime: ctime, utime: utime, lastAccess: lastAccess)
        }
        public static func fromRustTagResult(
            t: TagResult,
            existingTags: [TagModel]
        ) -> TagModel {
            let bl = t.body.lowercased()
            let exists = existingTags.first(where: {
                $0.value.lowercased() == bl
            })
            return TagModel(
                value: t.body,
                ctime: exists == nil ? .now : exists!.ctime,
                utime: .now,
                lastAccess: exists == nil ? .now : exists!.lastAccess
            )
        }
    }

    @available(iOS 26, *)
    @Model
    public class SubjectModel: TaggableModel {
        @Attribute(.unique) public var value: String
        @Relationship(inverse: \NoteModel.subject) public var notes: [NoteModel] = []
        public init(
            value: String,
            ctime: Date = .now,
            utime: Date = .now,
            lastAccess: Date = .now,
            notes: [NoteModel] = []
        ) {
            self.notes = notes
            self.value = value
            super.init(ctime: ctime, utime: utime, lastAccess: lastAccess)
        }
    }

    @available(iOS 26, *)
    @Model
    public class TopicModel: TaggableModel {
        @Attribute(.unique) public var value: String
        @Relationship(inverse: \NoteModel.topic) public var notes: [NoteModel] =
            []
        public init(
            value: String,
            ctime: Date = .now,
            utime: Date = .now,
            lastAccess: Date = .now,
            notes: [NoteModel] = []
        ) {
            self.value = value
            self.notes = notes
            super.init(ctime: ctime, utime: utime, lastAccess: lastAccess)
        }
    }

    @Model
    public class MarkdownNote {
        public var _body: String
        /// This will be required later when building for architecture's that don't support rust since the parser is being written in rust. It can also be used for some performance improvements.
        public var preParsedBody: String?
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
        public func parsedBodyText(saveBodyText: Bool) async -> String {
            let t = MdxText(body: self.body)
            t.parse()
            if saveBodyText {
                self.preParsedBody = t.body
            }
            return t.body
        }
    }
}
