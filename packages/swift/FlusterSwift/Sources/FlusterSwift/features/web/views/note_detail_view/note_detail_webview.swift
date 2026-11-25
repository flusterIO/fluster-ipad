//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/5/25.
//

import FlusterData
import SwiftData
import SwiftUI
import WebKit

#if os(iOS)

  public struct NoteDetailWebview: View {
    @State private var webView: WKWebView = WKWebView(
      frame: CGRect(x: 0, y: 0, width: 1, height: 1), configuration: getConfig()
    )

    @Environment(\.modelContext) private var modelContext: ModelContext
    @Binding var fullScreenCover: MainFullScreenCover?
    @Binding var note: NoteModel
    @AppStorage(AppStorageKeys.embeddedCslFile.rawValue) private var cslFile: EmbeddedCslFileSwift =
      .apa
      @State private var showWebView: Bool = false

    public init(
      fullScreenCover: Binding<MainFullScreenCover?>,
      note: Binding<NoteModel>
    ) {
      self._fullScreenCover = fullScreenCover
      self._note = note
    }
    public var body: some View {
      IosWebviewContainer(
        implementation: .noteDetails,
        editingNote: Binding<NoteModel>.toOptional($note),
        webView: $webView,
        show: $showWebView,
        url: URL.embeddedFlusterUrl(
          folder: "note_detail_webview_ipad", fileName: "index_ipad.html"),
        messageHandlerKeys: [
          NoteDetailWebviewActions.requestNoteDetailData.rawValue,
          NoteDetailWebviewActions.onTagClick.rawValue,
          NoteDetailWebviewActions.onTopicClick.rawValue,
          NoteDetailWebviewActions.onSubjectClick.rawValue,
          MdxPreviewWebviewActions.onCitationClick.rawValue
        ],
        messageHandler:
          self.messageHandler,
        onLoad: onLoad,
        srcUrl: "/note_detail_webview_ipad"
      )
      // onLoad only runs once, when the webview first loads. Re-push the details whenever the
      // note is switched or its displayed metadata changes.
      .onChange(of: detailsSignature) {
        if showWebView {
          Task { await onLoad() }
        }
      }
    }
    private var detailsSignature: String {
      [
        note.id,
        note.getPreferedTitle(),
        note.topic?.value ?? "",
        note.subject?.value ?? "",
        note.tags.map(\.value).joined(separator: ","),
        note.citations.compactMap(\.citationKey).joined(separator: ","),
        note.frontMatter.summary?.body ?? ""
      ].joined(separator: "|")
    }
    func messageHandler(_ messageKey: String, _ messageBody: Any) {
      switch messageKey {
        case NoteDetailWebviewActions.requestNoteDetailData.rawValue:
          Task(priority: .high) { await onLoad() }
        case NoteDetailWebviewActions.onTagClick.rawValue:
          guard let value = messageBody as? String else { return }
          if let tag = fetchFirst(TagModel.self, #Predicate<TagModel> { $0.value == value }) {
            fullScreenCover = .tagSearch(tag: tag)
          }
        case NoteDetailWebviewActions.onTopicClick.rawValue:
          guard let value = messageBody as? String else { return }
          if let topic = fetchFirst(TopicModel.self, #Predicate<TopicModel> { $0.value == value }) {
            fullScreenCover = .topicSearch(topic: topic)
          }
        case NoteDetailWebviewActions.onSubjectClick.rawValue:
          guard let value = messageBody as? String else { return }
          if let subject = fetchFirst(
            SubjectModel.self, #Predicate<SubjectModel> { $0.value == value })
          {
            fullScreenCover = .subjectSearch(subject: subject)
          }
        case MdxPreviewWebviewActions.onCitationClick.rawValue:
          guard let key = messageBody as? String else { return }
          if let entry = fetchFirst(
            BibEntryModel.self, #Predicate<BibEntryModel> { $0.citationKey == key })
          {
            fullScreenCover = .citationByKey(citation: entry)
          }
        default:
          return
      }
    }
    private func fetchFirst<T: PersistentModel>(_ type: T.Type, _ predicate: Predicate<T>) -> T? {
      var descriptor = FetchDescriptor<T>(predicate: predicate)
      descriptor.fetchLimit = 1
      return try? modelContext.fetch(descriptor).first
    }
    func onLoad() async {
      let dateFormatter = RelativeDateTimeFormatter()
      dateFormatter.unitsStyle = .full
      dateFormatter.dateTimeStyle = .named
      let lastModified = dateFormatter.localizedString(
        for: note.utime,
        relativeTo: .now
      )
      let lastRead = dateFormatter.localizedString(
        for: note.lastRead,
        relativeTo: .now
      )
    
        var idx: UInt32 = 0
        let citations = note.citations.compactMap { cit in
          idx += 1
          return cit.toEditorCitation(activeCslFile: cslFile, idx: idx)
        }

      let tags = note.tags.map { t in
        return EditorTag(body: t.value)
      }

      do {
        try await EditorState.setNoteDetails(
          payload: NoteDetailState(
            note_id: note.id,
            title: note.getPreferedTitle(),
            summary: note.frontMatter.summary?.toSummaryState(),
            topic: note.topic?.value,
            subject: note.subject?.value,
            tags: tags,
            citations: citations,
            last_modified_string: lastModified,
            last_read_string: lastRead),
          eval: self.webView.evaluateJavaScript
        )
      } catch {
        print("Error: \(error.localizedDescription)")
      }
    }
  }
#endif
