//
//  note_detail_webview.swift
//  Fluster
//
//  Created by Andrew on 2/17/26.
//

import FlusterData
import FlusterSwift
import FlusterWebviewClients
import SwiftData
import SwiftUI
import WebKit

struct NoteDetailWebview: View {
  let editingNoteId: String?
  @State private var webView: WKWebView = WKWebView(
    frame: .infinite, configuration: getWebViewConfig()
  )
  @Query var notes: [NoteModel]
  @AppStorage(AppStorageKeys.embeddedCslFile.rawValue) private var cslFile: EmbeddedCslFileSwift =
    .apa
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
        implementation: WebviewImplementation.noteDetails,
        editingNoteId: editingNoteId,
        webview: $webView,
        url: URL.embeddedFlusterUrl(folder: "note_detail_webview_mac", fileName: "index_mac.html"),
        messageHandlerKeys: [
          NoteDetailWebviewActions.requestNoteDetailData.rawValue,
          NoteDetailWebviewActions.onTagClick.rawValue,
          NoteDetailWebviewActions.onTopicClick.rawValue,
          NoteDetailWebviewActions.onSubjectClick.rawValue,
          MdxPreviewWebviewActions.onCitationClick.rawValue
        ],
        messageHandler: messageHandler,
        onLoad: onLoad,
        mathjaxFontUrl: "/note_detail_webview_mac"
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

  func handleCitationClick(citationKey: String) {
      print("Citation Id: \(citationKey)")
    var descriptor = FetchDescriptor<BibEntryModel>(
      predicate: #Predicate<BibEntryModel> { entry in
        entry.citationKey == citationKey
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
    case MdxPreviewWebviewActions.onCitationClick.rawValue:
        self.handleCitationClick(citationKey: msgBody as! String)
      default:
        return
    }
  }

  func onLoad() async {
    print("Setting note details")
    if let en = editingNote {
      let dateFormatter = RelativeDateTimeFormatter()
      dateFormatter.unitsStyle = .full
      dateFormatter.dateTimeStyle = .named
      let lastModified = dateFormatter.localizedString(
        for: en.utime,
        relativeTo: .now
      )
      let lastRead = dateFormatter.localizedString(
        for: en.lastRead,
        relativeTo: .now
      )
      let citations = en.citations.compactMap { cit in
        return cit.toEditorCitation(activeCslFile: cslFile)
      }
      let tags = en.tags.map { t in
        return EditorTag(body: t.value)
      }

      do {
        try await EditorState.setNoteDetails(
          payload: NoteDetailState(
            note_id: en.id, title: en.getPreferedTitle(), summary: en.frontMatter.summary?.body,
            topic: en.topic?.value, subject: en.subject?.value, tags: tags, citations: citations,
            last_modified_string: lastModified, last_read_string: lastRead),
          eval: self.webView.evaluateJavaScript
        )
      } catch {
        print("Error: \(error.localizedDescription)")
      }
    }
  }
}

#Preview {
  NoteDetailWebview(editingNoteId: nil)
}
