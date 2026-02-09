//
//  view_paper_page_view.swift
//  Fluster
//
//  Created by Andrew on 1/20/26.
//

import FlusterData
import SwiftData
import SwiftUI

struct ViewPaperPageView: View {
  @EnvironmentObject private var appState: AppState
  @Environment(\.modelContext) private var modelContext
  var editingNote: NoteModel? {
    guard let id = appState.editingNoteId else { return nil }
    return modelContext.model(for: id) as? NoteModel
  }

  var body: some View {
    if editingNote != nil {
      Text("View Paper")
    } else {
      NoNoteSelectedView()
    }
  }
}

#Preview {
  ViewPaperPageView()
}
