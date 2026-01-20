//
//  view_editing_note_mdx_page.swift
//  Fluster
//
//  Created by Andrew on 1/19/26.
//

import FlusterData
import SwiftUI

struct ViewEditingNoteMdxPage: View {
  @Environment(AppState.self) private var appState: AppState
  var body: some View {
    if let editingNote = appState.editingNote {
      ViewMdxNoteView(item: editingNote)
    } else {
      NoNoteSelectedView()
    }
  }
}

#Preview {
  ViewEditingNoteMdxPage()
}
