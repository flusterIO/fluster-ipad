//
//  edit_mdx_page_view.swift
//  Fluster Desktop
//
//  Created by Andrew on 1/15/26.
//

import SwiftUI
import WebKit

struct EditMdxPageView: View {
  @Environment(AppState.self) private var appState: AppState
  @Binding var webview: WKWebView

  var body: some View {
    if let editingNote = appState.editingNote {
      MdxEditorWebview(editingNote: editingNote, webView: $webview)
    } else {
      NoNoteSelectedView()
    }
  }
}

#Preview {
  EditMdxPageView(
    webview: .constant(
      WKWebView(
        frame: .infinite, configuration: getWebViewConfig()
      )))
}
