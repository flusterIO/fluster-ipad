import FlatBuffers
import FlusterMdx
import FlusterSwiftMdxParser
import Foundation
import PencilKit
import SwiftData
import SwiftyBibtex

public enum AppSchemaV1: VersionedSchema {
  public static var models: [any PersistentModel.Type] {
    // Leaving off other models as they should be able to be inferred and this will simplify migration scripts I hope.
    [NoteModel.self, BibEntryModel.self]
  }
  public static let versionIdentifier: Schema.Version = .init(1, 0, 0)
}

extension AppSchemaV1 {
  @Model
  public class FrontMatter {
    public var id: String
    public var title: String?
    public var userDefinedId: String?
    public var ignoreParsers: [String] = []

    init(id: String, title: String? = nil, userDefinedId: String? = nil) {
      self.id = id
      self.title = title
      self.userDefinedId = userDefinedId
    }

    public static func emptyFrontMatter() -> FrontMatter {
      return FrontMatter(
        id: UUID().uuidString,
        title: nil,
        userDefinedId: nil
      )
    }
    public func applyRustFrontMatterResult(
      res: MdxSerialization_FrontMatterResultBuffer
    ) {
      if res.title != nil, !res.title!.isEmpty {
        self.title = res.title
      }
      if res.userDefinedId != nil, !res.userDefinedId!.isEmpty {
        self.userDefinedId = res.userDefinedId
      }
    }
    public func applyParsingResult(
      res: FrontMatterResult
    ) {
      if res.title != nil, !res.title!.isEmpty {
        self.title = res.title
      }
      if res.userDefinedId != nil, !res.userDefinedId!.isEmpty {
        self.userDefinedId = res.userDefinedId
      }
      self.ignoreParsers = res.ignoredParsers
    }

    public static func fromParsingResult(data: FrontMatterResult)
      -> FrontMatter
    {
      let fm = FrontMatter(id: UUID.init().uuidString)
      fm.applyParsingResult(res: data)
      return fm
    }
  }
  public enum BibEntrySaveStrategy: String, Codable {
    case userAdded, fromNoteContent
  }
  @Model
  public class NoteModel {
    public var id: String
    @Attribute(.externalStorage) public var drawing: Data
    public var markdown: MarkdownNote
    public var frontMatter: FrontMatter = FrontMatter.emptyFrontMatter()
    public var ctime: Date
    public var utime: Date
    public var lastRead: Date
    public var subject: SubjectModel?
    public var topic: TopicModel?
    public var tags = [TagModel]()
    @Relationship(inverse: \BibEntryModel.notes)
    public var citations = [BibEntryModel]()
    @Relationship(deleteRule: .cascade, inverse: \DictionaryEntryModel.note)
    public var dictionaryEntries = [DictionaryEntryModel]()
    public var bookmarked: Bool = false
    ///  Must be a Map of `Map<citationKey ?? id, BibEntrySaveStrategy>`. The citationKey must be preferred first.
    public var bibEntryStrategyMap = [String: BibEntrySaveStrategy]()

    // drawing.toDataRepresentation() to conform to Data type.
    public init(
      id: String? = nil,
      drawing: Data = PKDrawing().dataRepresentation(),
      markdown: MarkdownNote = MarkdownNote(body: "", summary: nil),
      ctime: Date = .now,
      utime: Date = .now,
      lastRead: Date = .now,
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
      self.lastRead = lastRead
      self.subject = subject
      self.topic = topic
      self.tags = tags
      self.citations = citations
    }

    public func containsCitation(citation: BibEntryModel) -> Bool {
      return self.citations.contains(where: { $0.id == citation.id })
    }

    public func addCitation(citation: BibEntryModel, strategy: BibEntrySaveStrategy) {
      self.bibEntryStrategyMap[citation.citationKey ?? citation.id] = strategy
      self.citations.appendWithoutDuplicates(item: citation)
    }

    public func removeCitation(citation: BibEntryModel) {
      self.bibEntryStrategyMap.removeValue(forKey: citation.citationKey ?? citation.id)
      self.citations = self.citations.filter({
        $0.id != citation.id
      })
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
      results: MdxSerialization_MdxParsingResultBuffer,
      modelContext: ModelContext
    ) {
      self.markdown.preParsedBody = results.parsedContent
      if let frontMatter = results.frontMatter {
        self.frontMatter.applyRustFrontMatterResult(res: frontMatter)
      }
      var tags: [TagModel] = []
      for idx in (0..<results.tagsCount) {
        if let tag = results.tags(at: idx) {
          if let existingResult = self.tags.first(where: {
            $0.value == tag.body
          }) {
            tags.append(existingResult)
          } else {
            tags.append(TagModel(value: tag.body))
          }
        }
      }
      self.tags = tags
      // -- Citations --
      var citations: [BibEntryModel] = []
      let citationFetchDescriptor = FetchDescriptor<BibEntryModel>()
      let allCitations = try! modelContext.fetch(citationFetchDescriptor)
      // Save the bibEntries that were use defined since they cannot be automatically inferred from the note.
      for (citationId, saveStrategy) in self.bibEntryStrategyMap {
        if saveStrategy == .userAdded {
          if let existingCitation = self.citations.first(where: { cit in
            return cit.id == citationId
          }) {
            citations.append(existingCitation)
          }
        } else {
          // Remove all of the bibEntries that can be re-generated from the user's note content.
          self.bibEntryStrategyMap.removeValue(forKey: citationId)
        }
      }
      // Handle saving of additional bibEntries that can be generated from the note.
      let citationsLength = results.citationsCount
      //      var parsingResultCitations: [MdxSerialization_CitationResultBuffer] = []
      for idx in (0..<citationsLength) {
        if let citationItem = results.citations(at: idx) {
          if let existingCitation = allCitations.first(where: { cit in
            cit.citationKey == citationItem.citationKey
          }) {
            // Citation exists in datbase, can continue saving it
            citations.append(existingCitation)
            self.bibEntryStrategyMap[existingCitation.citationKey ?? existingCitation.id] =
              .fromNoteContent
          }
        }
      }

      self.citations = citations

      // -- Dictionary --
      var dictionayEntries: [DictionaryEntryModel] = []
      for idx in (0..<results.dictionaryEntriesCount) {
        if let dEntry = results.dictionaryEntries(at: idx) {
          if let existingEntry = self.dictionaryEntries.first(where: {
            $0.label == dEntry.label
          }) {
            dictionaryEntries.append(existingEntry)
          } else {
            dictionayEntries.append(
              DictionaryEntryModel(
                id: UUID.init().uuidString,
                label: dEntry.label,
                body: dEntry.body)
            )
          }
        }
      }
      self.dictionaryEntries = dictionayEntries
    }
    public func diassociateBibEntry(bibEntry: BibEntryModel) {
      self.citations.removeAll {
        $0.id == bibEntry.id
      }
    }
    public func setLastRead(setModified: Bool = false) {
      self.lastRead = .now
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
      if setModified {
        self.utime = .now
      }
    }

    public func getEditingNoteId() -> EditingNoteId {
      if let fmId = self.frontMatter.userDefinedId {
        return EditingNoteId.fromUserDefinedId(userDefinedId: fmId)
      } else {
        return EditingNoteId(value: self.id)
      }
    }
    public static func getEmptyModel(
      noteBody: String = "",
      noteSummary: String? = nil
    ) -> NoteModel {
      NoteModel(
        drawing: PKDrawing().dataRepresentation(),
        markdown: MarkdownNote(body: noteBody, summary: noteSummary)
      )
    }

    public static func fromInitialDataParsingResult(
      data: MdxParsingResult,
      existingTags: [TagModel],
      existingCitations: [BibEntryModel],
      forceUserDefinedCitations: Bool = false
    ) -> NoteModel {
      let note = NoteModel.getEmptyModel(
        noteBody: data.content,
        noteSummary: nil
      )
      if forceUserDefinedCitations {
        for cit in existingCitations {
          note.addCitation(citation: cit, strategy: .userAdded)
        }
      } else {
        for cit in data.citations {
          if let foundEntry = existingCitations.first(where: { citItem in
            citItem.citationKey == cit.citationKey
          }) {
            let citationAlreadyAppended = note.citations.contains(where: { alreadyAppendedCit in
              alreadyAppendedCit.citationKey == foundEntry.citationKey
            })
            if !citationAlreadyAppended {
              note.addCitation(citation: foundEntry, strategy: .fromNoteContent)
            }
          }
        }
      }
      note.tags = data.tags.map({ tag in
        TagModel.fromRustTagResult(t: tag, existingTags: existingTags)
      })
      if let fm = data.frontMatter {
        note.frontMatter = FrontMatter.fromParsingResult(data: fm)
      }

      return note
    }
  }
  @Model
  public class DictionaryEntryModel: Identifiable {
    @Attribute(.unique) public var id: String
    @Attribute(.unique) public var label: String
    public var body: String
    public var ctime: Date
    public var note: NoteModel?

    public init(
      id: String, label: String, body: String, note: NoteModel? = nil, ctime: Date = .now
    ) {
      self.id = id
      self.label = label
      self.body = body
      self.ctime = ctime
      self.note = note
    }
  }
  @Model
  public class BibEntryModel: Identifiable {
    @Attribute(.unique) public var id: String
    @Attribute(.unique) public var citationKey: String?
    public var htmlFormattedCitation: String?
    // swiftlint:disable:next identifier_name
    public var _data: String
    public var ctime: Date
    public var utime: Date
    /// The time a note with this bibliography entry was last accessed.
    public var lastAccess: Date
    public var notes = [NoteModel]()
    public var title: String
    public init(
      id: String? = nil,
      data: String,
      ctime: Date = .now,
      utime: Date = .now,
      lastAccess: Date = .now,
      notes: [NoteModel] = []
    ) {
      self.id = id ?? UUID().uuidString
      self._data = data
      self.ctime = ctime
      self.utime = utime
      self.notes = notes
      self.lastAccess = lastAccess
      // TODO: Figure out how to get the html formatted citation in rust and use that to generate
      //  a formatted citation for each item that can then be passed to the webview for bibliography creation.
      self.htmlFormattedCitation = nil
      self.title = ""  // Required to avoid an 'using self before all properties are initialized' error
      self.citationKey = self.getCitationKey()
      self.title = self.getTitle()
    }

    /// The bibtex string representing the item's data.
    public var data: String {
      get {
        return self._data
      }
      set(newData) {
        self._data = newData
        self.title = self.getTitle()
      }
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
      let result = try? SwiftyBibtex.parse(self.data)
      if result != nil {
        for warning in result!.warnings {
          print(warning.message)
        }
        if result!.publications.count == 1 {
          return result!.publications[0].fields["title"]
            ?? "No title found"
        }
      }
      return "No title found"
    }

    public func toCitationSummary() -> SwiftDataCitationSummary {
      return SwiftDataCitationSummary(
        citationKey: self.id,
        body: self.data
      )
    }
  }
  @Model
  public final class TagModel {
    @Attribute(.unique) public var value: String
    /// This is a hack to allow searching the DB with a case insensitive string.
    /// It will store duplicate data, but a tag should be pretty short anyways.
    /// This will also make sure the case insensitive string is unique.
    @Attribute(.unique) public var caseInsensitive: String
    @Relationship(inverse: \NoteModel.tags) public var notes: [NoteModel] =
      []
    public var ctime: Date
    public var utime: Date
    /// lastAccess is (will be) updated each time a note is accessed that contains a given taggable to help with search ranking.
    public var lastAccess: Date
    public init(
      value: String,
      ctime: Date = .now,
      utime: Date = .now,
      lastAccess: Date = .now,
      notes: [NoteModel] = []
    ) {
      self.value = value
      self.caseInsensitive = value.lowercased()
      self.notes = notes
      self.ctime = ctime
      self.utime = utime
      self.lastAccess = lastAccess
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

  @Model
  public final class SubjectModel {
    @Attribute(.unique) public var value: String
    /// This is a hack to allow searching the DB with a case insensitive string. It will store duplicate data, but a tag should be pretty short anyways.
    /// This will also make sure the case insensitive string is unique.
    @Attribute(.unique) public var caseInsensitive: String
    @Relationship(inverse: \NoteModel.subject) public var notes: [NoteModel] = []

    public var ctime: Date
    public var utime: Date
    /// lastAccess is (will be) updated each time a note is accessed that contains a given taggable to help with search ranking.
    public var lastAccess: Date
    public init(
      value: String,
      ctime: Date = .now,
      utime: Date = .now,
      lastAccess: Date = .now,
      notes: [NoteModel] = []
    ) {
      self.notes = notes
      self.value = value
      self.caseInsensitive = value.lowercased()
      self.ctime = ctime
      self.utime = utime
      self.lastAccess = lastAccess
    }
  }

  @Model
  public final class TopicModel {
    @Attribute(.unique) public var value: String
    /// This is a hack to allow searching the DB with a case insensitive string. It will store duplicate data, but a tag should be pretty short anyways.
    /// This will also make sure the case insensitive string is unique.
    @Attribute(.unique) public var caseInsensitive: String
    @Relationship(inverse: \NoteModel.topic) public var notes: [NoteModel] =
      []

    public var ctime: Date
    public var utime: Date
    /// lastAccess is (will be) updated each time a note is accessed that contains a given taggable to help with search ranking.
    public var lastAccess: Date
    public init(
      value: String,
      ctime: Date = .now,
      utime: Date = .now,
      lastAccess: Date = .now,
      notes: [NoteModel] = []
    ) {
      self.value = value
      self.caseInsensitive = value.lowercased()
      self.notes = notes
      self.ctime = ctime
      self.utime = utime
      self.lastAccess = lastAccess
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
  }

  // -- Desktop Stuff --
  @Model
  public class AutoTaggable {
    public var glob: String
    public var value: String
    public var settingType: AutoTaggableType

    public init(glob: String, value: String, settingType: AutoTaggableType) {
      self.glob = glob
      self.value = value
      self.settingType = settingType
    }
  }
}
