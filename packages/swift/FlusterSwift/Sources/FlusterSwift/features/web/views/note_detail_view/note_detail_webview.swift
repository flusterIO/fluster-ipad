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

    @Binding var fullScreenCover: MainFullScreenCover?
    @Binding var note: NoteModel
    @AppStorage(AppStorageKeys.embeddedCslFile.rawValue) private var cslFile: EmbeddedCslFileSwift =
      .apa

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
        url: URL.embeddedFlusterUrl(
          folder: "note_detail_webview_ipad", fileName: "index_ipad.html"),
        messageHandlerKeys: [],
        messageHandler:
          self.messageHandler,
        onLoad: onLoad
      )
    }
    func messageHandler(_ messageKey: String, _ messageBody: Any) {
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
      let citations = note.citations.compactMap { cit in
        return cit.toEditorCitation(activeCslFile: cslFile)
      }
      let tags = note.tags.map { t in
        return EditorTag(body: t.value)
      }

      do {
        try await EditorState.setNoteDetails(
          payload: NoteDetailState(
            note_id: note.id,
            title: note.getPreferedTitle(),
            summary: note.frontMatter.summary?.body,
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
