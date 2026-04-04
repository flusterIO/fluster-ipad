import ConundrumSwift
import FlatBuffers
import FlusterBibliography
import FlusterMdx
import Foundation
import PaperKit
import PencilKit
import SwiftData

public let DEFAULT_NOTE_TITLE: String = "No title found"

public enum AppSchemaV1: VersionedSchema {
  public static var models: [any PersistentModel.Type] {
    // Leaving off other models as they should be able to be inferred and this will simplify migration scripts I hope.
    return [
      NoteModel.self,
      BibEntryModel.self,
      AutoTaggable.self,
      NoteSummary.self,
      FrontMatter.self,
      Equation.self
    ]
  }
  public static let versionIdentifier: Schema.Version = .init(1, 0, 0)
}

extension AppSchemaV1 {
  public enum NoteSummaryGenerationMethod: String, Codable {
    case frontMatter, localAi, localAiManualTrigger, subtitleSyntax
    public func toSummaryGenerationMethod() -> SummaryGenerationMethod {
      switch self {
        case .frontMatter:
          return .frontMatter
        case .localAi:
          return .ai
        case .localAiManualTrigger:
          return .ai
        case .subtitleSyntax:
          return .subtitleSyntax
      }
    }
    public static func fromSummaryGenerationMethod(_ method: SummaryGenerationMethod) -> Self {
      switch method {
        case .aIManualTrigger:
          .localAiManualTrigger
        case .ai:
          .localAi
        case .frontMatter:
          .frontMatter
        case .subtitleSyntax:
          .subtitleSyntax
      }
    }
  }
  @Model
  final public class NoteSummary {
    public var generationMethod: NoteSummaryGenerationMethod
    public var body: String
    public var ctime: Date
    public var valid: Bool = true
    public init(
      generationMethod: NoteSummaryGenerationMethod,
      body: String,
      ctime: Date
    ) {
      self.generationMethod = generationMethod
      self.body = body
      self.ctime = ctime
    }

    public func toSummaryState() -> FlusterData.SummaryState {
      FlusterData.SummaryState(
        content: self.body, ctime: Float(self.ctime.timeIntervalSince1970.magnitude),
        generation_method: self.generationMethod.toSummaryGenerationMethod())
    }
  }
  @Model
  final public class FrontMatter {
    public var id: String
    public var title: String?
    public var userDefinedId: String?
    public var fsPath: String?
    public var ignoreParsers: [String] = []
    @Relationship(deleteRule: .cascade)
    public var summary: NoteSummary? = nil

    init(
      id: String,
      title: String? = nil,
      fsPath: String? = nil,
      userDefinedId: String? = nil,
      summary: NoteSummary? = nil
    ) {
      self.id = id
      self.title = title
      self.fsPath = fsPath
      self.userDefinedId = userDefinedId
      self.summary = summary
    }

    public static func emptyFrontMatter() -> FrontMatter {
      return FrontMatter(
        id: UUID().uuidString,
        title: nil,
        fsPath: nil,
        userDefinedId: nil,
        summary: nil
      )
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
      if let summary = res.summary {
        self.summary = NoteSummary(generationMethod: .frontMatter, body: summary, ctime: .now)
      }
    }

    public static func fromParsingResult(data: FrontMatterResult)
      -> FrontMatter
    {
      let fm = FrontMatter(id: UUID.init().uuidString)
      fm.applyParsingResult(res: data)
      return fm
    }

    public func applyAIGeneratedSummary(
      content: String, generationMethod: NoteSummaryGenerationMethod
    ) {
      if let existingSummaryMethod = self.summary?.generationMethod {
        if existingSummaryMethod == .frontMatter {
          return
        }
      }

      self.summary = NoteSummary(generationMethod: .localAi, body: content, ctime: .now)
    }
  }
  public enum BibEntrySaveStrategy: String, Codable {
    case userAdded, fromNoteContent
  }
  @Model
  public class PaperModel {
    @Attribute(.externalStorage) public var drawing: Data
    @Attribute(.externalStorage) public var markup: Data
    /// Tage page number, but 0 based.
    public var pageIndex: Int
    //    @Attribute(.externalStorage) public var thumbnail: CGImage?
    public init(
      markup: Data,
      drawing: Data = PKDrawing().dataRepresentation(),
      pageIndex: Int
    ) {
      self.markup = markup
      self.drawing = drawing
      self.pageIndex = pageIndex
    }
    //    public func generateThumbnail() async {
    //      let thumbnailSize = CGSize(width: 200, height: 200)
    //      let size = CGSize(width: 200, height: 200)
    //      if let context = CGContext(width: 200, height: 200) {
    //        context.setFillColor(gray: 1, alpha: 1)
    //        //            See https://www.youtube.com/watch?v=A31vmupv1eo at 8:15 to figure this out...
    //        //            context.fill(renderer.format.bounds)
    //        await self.markup.draw(in: context, frame: CGRect(origin: .zero, size: size))
    //        thumbnail = context.makeImage()
    //      }
    //    }
  }

  public enum R3AxisId: String, Codable {
    case x, y, z
  }

  /// A label and description of an axis of the 3-d
  /// @param desc: required to be fed to the AI.
  /// @param label: Text displayed on plot
  @Model
  public class R3AxisData {
    public var label: String
    public var desc: String
    public var R3AxisId: R3AxisId
    /// A value between 0..1. The vector is nto necessarily normalized, but the vector of any
    /// invidual component should not exceed 1.
    public var value: Float
    public var valid: Bool = true
    public init(label: String, desc: String, value: Float, R3AxisId: R3AxisId) {
      self.label = label
      self.desc = desc
      self.R3AxisId = R3AxisId
      self.value = value
    }
  }

  @Model
  /// A R3 vector used to plot the note in 3-d space. Kind of a hack to use the minimal onboard
  /// AI as a visual vector store since it defiitely can't handle 768 dimensions.
  public class NoteVec3 {
    public var x: R3AxisData
    public var y: R3AxisData
    public var z: R3AxisData
    public var ctime: Date

    public init(x: R3AxisData, y: R3AxisData, z: R3AxisData, ctime: Date = .now) {
      self.x = x
      self.y = y
      self.z = z
      self.ctime = ctime
    }
  }

  @Model
  public class NoteModel {
    public var id: String
    public var paper = [PaperModel]()
    //    @Attribute(.externalStorage) public var drawing: Data
    public var markdown: MarkdownNote
    public var frontMatter: FrontMatter
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
    /// Future proofing for now.
    public var protected: Bool = false
    public var r3_vec: NoteVec3
    /// Not yet being populated, but being setup to avoid versioning issues when the app moves back towards
    /// the version that helped me.
    public var equations: [Equation]

    /// drawing.toDataRepresentation() to conform to Data type.
    public init(
      id: String? = nil,
      drawing: [PaperModel] = [PaperModel](),
      markdown: MarkdownNote = MarkdownNote(body: ""),
      ctime: Date = .now,
      utime: Date = .now,
      lastRead: Date = .now,
      subject: SubjectModel? = nil,
      topic: TopicModel? = nil,
      tags: [TagModel] = [TagModel](),
      citations: [BibEntryModel] = [BibEntryModel](),
      r3_vec: NoteVec3 = NoteVec3(
        x: R3AxisData(label: "X", desc: "Math & Physics", value: 0, R3AxisId: .x),
        y: R3AxisData(
          label: "Y", desc: "Personal Notes, Journaling and Unstructured Notes", value: 0,
          R3AxisId: .y),
        z: R3AxisData(
          label: "Z", desc: "Business, Health & Personal Planning", value: 0, R3AxisId: .z)),
      equations: [Equation] = []
    ) {
      self.id = id ?? UUID.init().uuidString
      self.paper = drawing
      self.markdown = markdown
      self.ctime = ctime
      self.utime = utime
      self.lastRead = lastRead
      self.subject = subject
      self.topic = topic
      self.tags = tags
      self.citations = citations
      self.r3_vec = r3_vec
      self.frontMatter = FrontMatter.emptyFrontMatter()
      self.equations = equations
    }

    public func containsCitation(citation: BibEntryModel) -> Bool {
      return self.citations.contains(where: { $0.id == citation.id })
    }

    /// Appends a citation to the citation list, replacing an item by citationKey if that item already exists.
    ///  This leads to a behavior where any new bibliography entry with a given citationKey will override
    ///  a previous entry if they share the same citationKey.
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
      let title = ConundrumTextUtils.getTitleGroupSync(
        content: noteBody, modifiers: [.forcePlainText])?.title
      return title == nil
        ? false : title!.localizedCaseInsensitiveContains(query)
    }

    public static func fromNoteBody(noteBody: String) -> NoteModel {
      let note = NoteModel(
        markdown: MarkdownNote(body: noteBody)
      )
      return note
    }
    public func applyMdxParsingResults(
      results: MdxParsingResult,
      modelContext: ModelContext
    ) {
      self.markdown.isEdited = false
      self.markdown.preParsedBody = results.content
      if let frontMatter = results.frontMatter {
        self.frontMatter.applyParsingResult(res: frontMatter)
      }
      var tags: [TagModel] = []
      for idx in (0..<results.tags.count) {
        let tag = results.tags[idx]
        if let existingResult = self.tags.first(where: {
          $0.value == tag.body
        }) {
          tags.append(existingResult)
        } else {
          tags.append(TagModel(value: tag.body))
        }
      }
      self.tags = tags
      // -- Citations --
      var citations: [BibEntryModel] = []
      let citationFetchDescriptor = FetchDescriptor<BibEntryModel>()
      let allCitations = try! modelContext.fetch(citationFetchDescriptor)
      // Make sure the bibEntries that were user defined are not deleted since they cannot be automatically inferred from the note.
      for (citationKey, saveStrategy) in self.bibEntryStrategyMap {
        if saveStrategy == .userAdded {
          if let existingCitation = allCitations.first(where: { cit in
            return cit.citationKey != nil && cit.citationKey == citationKey
          }) {
            citations.append(existingCitation)
          }
        } else {
          // Remove all of the bibEntries that can be re-generated from the user's note content.
          self.bibEntryStrategyMap.removeValue(forKey: citationKey)
        }
      }
      // Handle saving of additional bibEntries that can be generated from the note.
      let citationsLength = results.orderedCitationKeys.count
      //      var parsingResultCitations: [MdxSerialization_CitationResultBuffer] = []
      for idx in (0..<citationsLength) {
        let citationItem = results.orderedCitationKeys[idx]
        if let existingCitation = allCitations.first(where: { cit in
          cit.citationKey == citationItem
        }) {
          // Citation exists in datbase, can continue saving it
          citations.append(existingCitation)
          self.bibEntryStrategyMap[existingCitation.citationKey ?? existingCitation.id] =
            .fromNoteContent
        }
      }

      self.citations = citations

      // -- Dictionary --
      var dictionaryEntries: [DictionaryEntryModel] = []
      for idx in (0..<results.dictionaryEntries.count) {
        let dEntry = results.dictionaryEntries[idx]
        if let existingEntry = self.dictionaryEntries.first(where: {
          $0.label == dEntry.label
        }) {
          dictionaryEntries.append(existingEntry)
        } else {
          dictionaryEntries.append(
            DictionaryEntryModel(
              id: UUID.init().uuidString,
              label: dEntry.label,
              body: dEntry.body)
          )
        }
      }
      // Come back and improve this with a batch operation or by somehow avoiding this last loop all together when you have time.
      self.dictionaryEntries.forEach { existingEntry in
        if !dictionaryEntries.contains(where: { $0.label == existingEntry.label }) {
          modelContext.delete(existingEntry)
        }
      }

      self.dictionaryEntries = dictionaryEntries
      self.updateTitleGroup()
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
    ) -> NoteModel {
      NoteModel(
        markdown: MarkdownNote(body: noteBody)
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
      )
      if forceUserDefinedCitations {
        for cit in existingCitations {
          note.addCitation(citation: cit, strategy: .userAdded)
        }
      } else {
        for cit in data.orderedCitationKeys {
          if let foundEntry = existingCitations.first(where: { citItem in
            citItem.citationKey == cit
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
    @MainActor
    public static func fetchFirstById(id: String, modelContainer: ModelContainer) -> NoteModel? {
      let fetchDescriptor = FetchDescriptor<NoteModel>(
        predicate: #Predicate<NoteModel> { note in
          note.id == id
        }
      )
      do {
        let data = try modelContainer.mainContext.fetch(fetchDescriptor)
        return data.isEmpty ? nil : data.first
      } catch {
        print("Error fetching note: \(error.localizedDescription)")
        return nil
      }
    }

    @MainActor
    public func preParse(modelContext: ModelContext) async throws {
      let _id = self.id
      let _body = self.markdown.body
      let task: Task<MdxParsingResult?, any Error> = try await Task.detached {
        let res = try await ConundrumSwift.runConundrum(
          options: ParseMdxOptions(noteId: _id, content: _body, modifiers: []))
        return res
      }

      let res = await task.result

      if let parsedMdx = try await res.get() {
        self.applyMdxParsingResults(
          results: parsedMdx,
          modelContext: modelContext
        )
      }  // No need for else block, get() already throws.
    }
    @MainActor
    public func preParseIfEdited(modelContext: ModelContext) async throws {
      if self.markdown.isEdited || self.markdown.preParsedBody == nil {
        try await self.preParse(modelContext: modelContext)
      }
    }
    @MainActor
    public func preParsedOrParse(modelContext: ModelContext) async throws -> String {
      if let preParsed = self.markdown.preParsedBody {
        return preParsed
      } else {
        try await self.preParse(modelContext: modelContext)
        return self.markdown.preParsedBody ?? ""
      }
    }
    public func toNoteDetailsByteArray() -> [UInt8] {
      var builder = FlatBufferBuilder(initialSize: 1024)
      var noteIdOffset = builder.create(string: self.id)
      var titleOffset = builder.create(
        string: self.markdown.title ?? "No title found"
      )
      var tagVectorOffset: [Offset] = []

      for t in self.tags {
        let x = MdxSerialization_TagResultBuffer.createTagResultBuffer(
          &builder,
          bodyOffset: builder.create(string: t.value)
        )
        tagVectorOffset.append(x)
      }

      var citationsVectorOffset: [Offset] = []

      for (idx, citation) in self.citations.enumerated() {
        let citationOffset =
          MdxSerialization_NoteDetails_NoteDetailCitationBuffer.createNoteDetailCitationBuffer(
            &builder,
            idOffset: builder.create(string: citation.citationKey ?? citation.id),
            bodyOffset: builder.create(string: citation.data),
            idx: UInt8(idx)
          )
        citationsVectorOffset.append(citationOffset)
      }

      let dateFormatter = RelativeDateTimeFormatter()
      dateFormatter.unitsStyle = .full
      dateFormatter.dateTimeStyle = .named
      let noteSummary: String? = self.frontMatter.summary?.body
      let details =
        MdxSerialization_NoteDetails_NoteDetailDataBuffer
        .createNoteDetailDataBuffer(
          &builder,
          noteIdOffset: noteIdOffset,
          titleOffset: titleOffset,
          summaryOffset: builder.create(string: noteSummary),
          topicOffset: builder.create(
            string: self.topic?.value
          ),
          subjectOffset: builder.create(
            string: self.subject?.value
          ),
          tagsVectorOffset: builder.createVector(
            ofOffsets: tagVectorOffset
          ),
          citationsVectorOffset: builder.createVector(
            ofOffsets: citationsVectorOffset
          ),
          lastModifiedStringOffset: builder.create(
            string: dateFormatter.localizedString(
              for: self.utime,
              relativeTo: .now
            )
          ),
          lastReadStringOffset: builder.create(
            string: dateFormatter.localizedString(
              for: self.lastRead,
              relativeTo: .now
            )
          )
        )
      builder.finish(offset: details)
      return builder.toBytesArray()
    }

    /// Returns a label for the note sufficient for use in 'title' use cases.
    /// This prefers front matter to markdown content if it exists.
    public func getPreferedTitle() -> String {
      return self.frontMatter.title ?? self.markdown.title ?? DEFAULT_NOTE_TITLE
    }

    /// pageIndex is the `pageIndex` field, not the actual index necessarily.
    public func removePaperByPageIndex(pageIndex: Int, modelContext: ModelContext) {
      var toKeep: [PaperModel] = []

      self.paper.forEach { p in
        if p.pageIndex == pageIndex {
          modelContext.delete(p)
        } else {
          toKeep.append(p)
        }
      }

      self.paper = toKeep
    }

    public func updateTitleGroup() {
      if let res = try? ConundrumTextUtils.getTitleGroupSync(
        content: self.markdown.body,
        modifiers: [.preferInlineMarkdownSyntax, .preferMarkdownSyntax])
      {
        self.markdown.title = res.title
        if let subtitle = res.subtitle, self.frontMatter.summary == nil {
          self.frontMatter.summary = NoteSummary(
            generationMethod: .subtitleSyntax, body: subtitle, ctime: .now)
          return
        }
      }
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
    public func toWebviewDictionaryEntry() -> WebviewDictionaryEntry {
      WebviewDictionaryEntry(label: self.label, body: self.body, origin_note_id: self.note?.id)
    }
  }

  public enum CitationUsage {
    case inlineText, fullBibliography
  }
  @Model
  public final class FormattedCitation {
    public var formattedHtml: String
    public var formattedPlainText: String
    public var cslFormatId: String
    public var bibEntry: BibEntryModel?
    public var title: String?
    public var note: String?
    public var abstract: String?
    public var url: String?

    public init(
      formattedHtml: String,
      formattedPlainText: String,
      cslFormatId: String,
      title: String?,
      note: String?,
      abstract: String?,
      url: String?,
      bibEntry: BibEntryModel?
    ) {
      self.formattedHtml = formattedHtml
      self.formattedPlainText = formattedPlainText
      self.cslFormatId = cslFormatId
      self.title = title
      self.note = note
      self.abstract = abstract
      self.url = url
      self.bibEntry = bibEntry
    }
  }
  @Model
  public final class ComponentData {
    /// The `id` field provided directly to the component by the user.
    @Attribute(.unique) public var id: String
    @Attribute(.externalStorage) public var data: Data
    /// Used for keeping track of potentially deleted components.
    public var lastRender: Date
    public init(id: String, data: Data, lastRender: Date) {
      self.id = id
      self.data = data
      self.lastRender = lastRender
    }
  }
  @Model
  public final class BibEntryModel: Identifiable {
    @Attribute(.unique) public var id: String
    @Attribute(.unique) public var citationKey: String?
    // swiftlint:disable:next identifier_name
    public private(set) var _data: String
    public var ctime: Date
    public var utime: Date
    /// The time a note with this bibliography entry was last accessed.
    public var lastAccess: Date
    public var notes = [NoteModel]()
    // title is required to be saved for alphabetizing.
    public var title = ""
    private var formatted: FormattedCitation?
    public init(
      id: String? = nil,
      data: String,
      ctime: Date = .now,
      utime: Date = .now,
      lastAccess: Date = .now,
      notes: [NoteModel]
    ) {
      self.id = id ?? UUID().uuidString
      self._data = data
      self.ctime = ctime
      self.utime = utime
      self.notes = notes
      self.lastAccess = lastAccess
      self.title = ""  // Required to avoid an 'using self before all properties are initialized' error
      self.citationKey = self.getCitationKey()
      self.title = getTitle()
    }

    /// The bibtex string representing the item's data.
    public var data: String {
      get {
        return self._data
      }
      set(newData) {
        self._data = newData
      }
    }

    public func toBiblatexData() -> BibEntryData? {
      let res = FlusterBibliography.parseBiblatexString(biblatexContent: self.data)
      if res.isEmpty {
        return nil
      } else {
        return res[0]
      }
    }

    public func toFormattedCitation(cslFormat: EmbeddedCslFileSwift) -> FormattedCitation? {
      if let entry = self.toBiblatexData() {
        let cslContent = cslFormat.toFlusterBibliographyCslFile()
        let localContent = getCslLocaleFileContent()
        let htmlFormatted = entry.formatBibliographyCitation(
          cslContent: cslContent, cslLocale: localContent, renderMethod: .html)
        let plainTextFormatted = entry.formatBibliographyCitation(
          cslContent: cslContent, cslLocale: localContent, renderMethod: .plaintext)
        let title = entry.getTitle()
        let abstract = entry.getAbstract()
        let url = entry.getUrl()
        let note = entry.getNote()
        if let formattedHtml = htmlFormatted, let formattedPlainText = plainTextFormatted {
          let formattedCitation = FormattedCitation(
            formattedHtml: formattedHtml, formattedPlainText: formattedPlainText,
            cslFormatId: cslFormat.rawValue,
            title: title,
            note: note,
            abstract: abstract,
            url: url,
            bibEntry: self
          )
          self.formatted = formattedCitation
          return formattedCitation
        }
      }
      return nil
    }

    public func parse() -> String? {
      //      let result = try? SwiftyBibtex.parse(self.data)
      //      if result != nil {
      //        for warning in result!.warnings {
      //          print(warning.message)
      //        }
      //        if result!.publications.count == 1 {
      //          return result
      //        }
      //      }
      return nil
    }

    public func getCitationKey() -> String? {
      self.toBiblatexData()?.getCitationKey()
    }

    public func getTitle() -> String {
      return self.toBiblatexData()?.getTitle() ?? "No title found"
    }

    public func toCitationSummary() -> CitationSummaryData {
      return CitationSummaryData(
        citationKey: self.id,
        body: self.data
      )
    }

    /// Gets the 'formattedCitation', parsing the new one only if the format has changed. This allows parsing the data on the 'data' change, and then being sure it's accurate for that data BibEntryModel the whole time the format.cslFormatId field matches the current csl format. This should speed up performance quite a bit on the bibliography pages.
    public func safelyGetFormatted(activeCslFormat: EmbeddedCslFileSwift) -> FormattedCitation? {
      if let currentFormattedCitiation = self.formatted,
        currentFormattedCitiation.cslFormatId == activeCslFormat.rawValue
      {
        return currentFormattedCitiation
      } else {
        let newlyFormattedCitation = self.toFormattedCitation(cslFormat: activeCslFormat, )
        self.formatted = newlyFormattedCitation
        return newlyFormattedCitation
      }
    }

    public func toFormattedCitation(
      _ usage: CitationUsage, _ renderMethod: RenderMethod, _ format: EmbeddedCslFileSwift
    ) -> String? {
      if usage == .fullBibliography {
        return self.toBiblatexData()?.formatBibliographyCitation(
          cslContent: format.toFlusterBibliographyCslFile(), cslLocale: getCslLocaleFileContent(),
          renderMethod: renderMethod)
      } else {
        return self.toBiblatexData()?.formatInlineCitation(
          cslContent: format.toFlusterBibliographyCslFile(), cslLocale: getCslLocaleFileContent(),
          renderMethod: renderMethod)
      }
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
  public class TocHeading {
    public var content: String
    public var depth: Int
    public var domId: String
    public var toc: NoteToc?
    public init(content: String, depth: Int, domId: String, toc: NoteToc?) {
      self.content = content
      self.depth = depth
      self.domId = domId
      self.toc = toc
    }
  }

  @Model
  public class NoteToc {
    @Relationship(deleteRule: .cascade, inverse: \TocHeading.toc)
    public var headings: [TocHeading]
    public var note: MarkdownNote?
    public init(headings: [TocHeading], note: MarkdownNote?) {
      self.headings = headings
      self.note = note
    }
  }

  @Model
  public class MarkdownNote {
    @Attribute(.externalStorage) public var _body: String
    @Attribute(.externalStorage) public var preParsedBody: String?
    public var title: String?
    /// The `titlePlainText` field is only populated when the plain text title does not match the  normal title for text comparison
    public var titlePlainText: String?
    /// Set to true every time the parsed state is changed to then update the `plainText` field.
    public var isEdited: Bool = false
    /// Set to nil by default, this is not parsed every time the mdx content is parsed for performance reasons, but is regenerated during downtime when the body changes.
    public var plainText: String?
    public var requiresPlainTextUpdate: Bool = false

    @Relationship(deleteRule: .cascade, inverse: \NoteToc.note)
    public var toc: NoteToc?

    public init(body: String) {
      self._body = body
    }

    public var body: String {
      get {
        return _body
      }
      set(newBody) {
        self._body = newBody
        self.isEdited = true
        self.requiresPlainTextUpdate = true
      }
    }

    // PERFORMANCE: Figure out how to move this to a background thread immediately.
    /// The noteId is the database id of the note, _not_ the user-defined id.
    @MainActor
    public func parsePlainText(noteId: String) async throws {
      let res = try await ConundrumSwift.runConundrum(
        options: ParseMdxOptions(noteId: noteId, content: self._body, modifiers: [.forcePlainText]))
      self.plainText = res.content
      if let title = self.title {
        let titleResponse = try await ConundrumSwift.runConundrum(
          options: ParseMdxOptions(noteId: noteId, content: title, modifiers: [.forcePlainText]))
        if titleResponse.content != title {
          self.titlePlainText = titleResponse.content
        } else {
          self.titlePlainText = nil
        }
      }
      self.requiresPlainTextUpdate = false
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

  @Model
  public class CodeSnippet {
    /// One of shiki's bundled languages
    public var language: String
    public var label: String?
    public var content: String
    public var equation: Equation?
    public init(language: String, content: String, label: String?, equation: Equation?) {
      self.language = language
      self.content = content
      self.label = label
      self.equation = equation
    }
  }

  @Model
  public class Equation {
    public var label: String
    public var desc: String?
    public var mathJaxContent: String
    public var codeContent: [CodeSnippet]
    public init(
      label: String, desc: String? = nil, mathJaxContent: String, codeContent: [CodeSnippet]
    ) {
      self.label = label
      self.desc = desc
      self.mathJaxContent = mathJaxContent
      self.codeContent = codeContent
    }
  }
}

public func getDevSchema() -> Schema {
  return Schema([
    NoteModel.self,
    BibEntryModel.self,
    AutoTaggable.self
  ])
}
