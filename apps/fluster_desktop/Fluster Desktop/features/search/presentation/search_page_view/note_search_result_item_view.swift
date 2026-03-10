//
//  note_search_result_item_view.swift
//  Fluster
//
//  Created by Andrew on 1/19/26.
//

import FlusterData
import FlusterSwift
import SwiftUI

struct NoteSearchResultItemView: View {
  let item: NoteModel
  @Binding var searchByTopic: TopicModel?
  @Binding var searchBySubject: SubjectModel?
  let dismissOnNavigate: Bool
  @EnvironmentObject private var appState: AppState
  @Environment(\.dismiss) private var dismiss
  @AppStorage(DesktopAppStorageKeys.defaultNoteView.rawValue) private var defaultNoteView:
    DefaultNoteView = .markdown
  var body: some View {
    VStack(alignment: .leading) {
      MarkdownTextView(item.getPreferedTitle(), .inlineOnly)
        .font(.title2)
      if let summary = item.frontMatter.summary?.body, !summary.isEmpty {
        Text(summary)
          .font(.subheadline)
          .foregroundStyle(.secondary)
      }
      NoteDashboardBottomRow(
        item: item, searchBySubject: $searchBySubject, searchByTopic: $searchByTopic)
    }
    .padding()
    .frame(maxWidth: .infinity, alignment: .leading)
    .glassEffect(in: .rect(cornerRadius: 12))
    .contentShape(Rectangle())
    .onTapGesture {
      appState.setEditingNote(editingNote: item)
      appState.mainView = defaultNoteView.toMainKey()
      if dismissOnNavigate {
        dismiss()
      }
    }
  }
}

#Preview {
  NoteSearchResultItemView(
    item: NoteModel.fromNoteBody(noteBody: "# My Note"),
    searchByTopic: .constant(nil),
    searchBySubject: .constant(nil),
    dismissOnNavigate: true
  )
}
