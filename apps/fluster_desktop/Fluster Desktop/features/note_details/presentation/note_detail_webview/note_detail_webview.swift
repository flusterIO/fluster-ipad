//
//  note_detail_webview.swift
//  Fluster
//
//  Created by Andrew on 2/17/26.
//

import FlusterData
import FlusterSwift
import SwiftData
import SwiftUI
import WebKit

struct NoteDetailWebview: View {
  let editingNoteId: String?
  @State private var webView: WKWebView = WKWebView(
    frame: .infinite, configuration: getWebViewConfig()
  )
  @Query var notes: [NoteModel]
  var editingNote: NoteModel? {
    notes.isEmpty ? nil : notes.first
  }
  init(editingNoteId: String?) {
    self.editingNoteId = editingNoteId
    if let _id = editingNoteId {
      var descriptor = FetchDescriptor(
        predicate: #Predicate<NoteModel> { note in
          note.id == _id
        }
      )
      descriptor.fetchLimit = 1
      self._notes = Query(
        descriptor
      )
    } else {
      var descriptor = FetchDescriptor(
        predicate: #Predicate<NoteModel> { note in
          false
        }
      )
      descriptor.fetchLimit = 0
      self._notes = Query()
    }
  }
  var body: some View {
    if editingNote != nil {
      WebViewContainerView(
        webview: $webView,
        url: Bundle.main.url(
          forResource: "index",
          withExtension: "html",
          subdirectory: "note_detail_webview"
        )!,
        messageHandlerKeys: [
          NoteDetailWebviewActions.requestNoteDetailData.rawValue,
          NoteDetailWebviewActions.onTagClick.rawValue,
          NoteDetailWebviewActions.onTopicClick.rawValue,
          NoteDetailWebviewActions.onSubjectClick.rawValue,
          NoteDetailWebviewActions.onCitationClick.rawValue
        ],
        messageHandler: messageHandler,
        onLoad: onLoad
      )
    } else {
      NoNoteSelectedView()
    }
  }

  func handleTagClick(tagBody: String) {
    print("Tag Body: \(tagBody)")
  }

  func handleTopicClick(topicValue: String) {
    print("Topic Value: \(topicValue)")
  }

  func handleSubjectClick(subjectValue: String) {
    print("Subject Value: \(subjectValue)")
  }

  func handleCitationClick(citationId: String) {
    print("Citation Id: \(citationId)")
  }

  func messageHandler(msgKey: String, msgBody: Any) {
    switch msgKey {
      case NoteDetailWebviewActions.requestNoteDetailData.rawValue:
        Task(priority: .high) {
          await self.onLoad()
        }
      case NoteDetailWebviewActions.onTagClick.rawValue:
        self.handleTagClick(tagBody: msgBody as! String)
      case NoteDetailWebviewActions.onTopicClick.rawValue:
        self.handleTopicClick(topicValue: msgBody as! String)
      case NoteDetailWebviewActions.onSubjectClick.rawValue:
        self.handleSubjectClick(subjectValue: msgBody as! String)
      case NoteDetailWebviewActions.onCitationClick.rawValue:
        self.handleCitationClick(citationId: msgBody as! String)
      default:
        return
    }
  }

  func onLoad() async {
    if let en = editingNote {
      let bytes = en.toNoteDetailsByteArray()
      do {
        try await webView.evaluateJavaScript(
          """
            window.dispatchEvent(
                new CustomEvent("\(NoteDetailWebviewEvents.setNoteDetails.rawValue)", {
                    detail: \(bytes),
                }),
            );
          """)
      } catch {
        print("Error: \(error.localizedDescription)")
      }
    }
  }
}

#Preview {
  NoteDetailWebview(editingNoteId: nil)
}
