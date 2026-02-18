//
//  create_bib_entry_view.swift
//  Fluster
//
//  Created by Andrew on 2/18/26.
//

import FlusterData
import FlusterWebviewClients
import SwiftUI
import WebKit

struct CreateBibEntryView: View {
  /// If editingItem != nil, the editingItem will be updated. Else, a new bibEntry will be created
  let editingItem: BibEntryModel?
  @State private var newItemData: String = ""
  /// If associateWithEditingNote == true, *new* BibEntryModel will be associated with the note currently being edited.
  let associateWithEditingNote: Bool = true
  @State private var webView: WKWebView = WKWebView(
    frame: .infinite, configuration: getWebViewConfig()
  )
  var body: some View {
    WebViewContainerView(
      webview: $webView,
      url: Bundle.main.url(
        forResource: "index",
        withExtension: "html",
        subdirectory: "bibtex_editor_webview"
      )!,
      messageHandlerKeys: [
        BibtexEditorWebviewActions.onEditorChange.rawValue,
        BibtexEditorWebviewActions.requestBibtexEditorData.rawValue,
        BibtexEditorWebviewActions.setWebviewLoaded.rawValue
      ],
      messageHandler: messageHandler,
      onLoad: onWebviewLoad
    )
  }
  func handleEditorChange(newValue: String) {
    if let ei = editingItem {
      ei.data = newValue
    }
  }
  func setEmptyEntryData() {
    Task {
      do {
        try await BibtexEditorClient.setBibEntryContent(
          entryBody: "", evalutateJavaScript: webView.evaluateJavaScript)
      } catch {
        print("Error setting bibliography editor data: \(error.localizedDescription)")
      }
    }
  }
  func setBibEntryData(data: BibEntryModel) {
    Task {
      do {
        try await BibtexEditorClient.setBibEntryContent(
          entryBody: data.data, evalutateJavaScript: webView.evaluateJavaScript)
      } catch {
        print("Error setting bibliography editor data: \(error.localizedDescription)")
      }
    }
  }
  func messageHandler(_ msgKey: String, msgBody: Any) {
    switch msgKey {
      case BibtexEditorWebviewActions.onEditorChange.rawValue:
        handleEditorChange(newValue: msgBody as! String)
      case BibtexEditorWebviewActions.requestBibtexEditorData.rawValue:
        Task(priority: .high) {
          await onWebviewLoad()
        }
      case BibtexEditorWebviewActions.setWebviewLoaded.rawValue:
        print("Webview Loaded...")
      default:
        return
    }
  }
  func onWebviewLoad() async {
    if let entry = editingItem {
      setBibEntryData(data: entry)
    } else {
        setEmptyEntryData()
    }
  }
}
