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
  @Environment(\.modelContext) private var modelContext: ModelContext
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
          forResource: "index_mac",
          withExtension: "html",
          subdirectory: "note_detail_webview_mac"
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
    var descriptor = FetchDescriptor<TagModel>(
      predicate: #Predicate<TagModel> { tag in
        tag.value == tagBody
      }
    )
    descriptor.fetchLimit = 1
    do {
      if let tag = try modelContext.fetch(descriptor).first {
        AppState.shared.commandPaletteNavigate(to: .searchByTag(tag))
      }
    } catch {
      print("Error: \(error.localizedDescription)")
    }
  }

  func handleTopicClick(topicValue: String) {
    var descriptor = FetchDescriptor<TopicModel>(
      predicate: #Predicate<TopicModel> { topic in
        topic.value == topicValue
      }
    )
    descriptor.fetchLimit = 1
    do {
      if let topic = try modelContext.fetch(descriptor).first {
        AppState.shared.commandPaletteNavigate(to: .searchByTopic(topic))
      }
    } catch {
      print("Error: \(error.localizedDescription)")
    }
  }

  func handleSubjectClick(subjectValue: String) {
    var descriptor = FetchDescriptor<SubjectModel>(
      predicate: #Predicate<SubjectModel> { subject in
        subject.value == subjectValue
      }
    )
    descriptor.fetchLimit = 1
    do {
      if let subject = try modelContext.fetch(descriptor).first {
        AppState.shared.commandPaletteNavigate(to: .searchBySubject(subject))
      }
    } catch {
      print("Error: \(error.localizedDescription)")
    }
  }

  func handleCitationClick(citationId: String) {
    var descriptor = FetchDescriptor<BibEntryModel>(
      predicate: #Predicate<BibEntryModel> { entry in
        entry.id == citationId
      }
    )
    descriptor.fetchLimit = 1
    do {
      if let bibEntry = try modelContext.fetch(descriptor).first {
        AppState.shared.commandPaletteNavigate(to: .searchByCitation(bibEntry))
      }
    } catch {
      print("Error: \(error.localizedDescription)")
    }
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
