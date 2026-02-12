//
//  note_dashboard_item.swift
//  Fluster
//
//  Created by Andrew on 1/19/26.
//

import FlusterData
import SwiftUI

struct NoteDashboardItem: View {
  let item: NoteModel
  @Binding var searchByTopic: TopicModel?
  @Binding var searchBySubject: SubjectModel?
  @EnvironmentObject private var appState: AppState
  @AppStorage(DesktopAppStorageKeys.defaultNoteView.rawValue) private var defaultNoteView:
    DefaultNoteView = .markdown
  var body: some View {
    HStack(alignment: .center) {
      Image(systemName: "text.document")
        .resizable()
        .frame(width: 24, height: 24)
        .padding(8)
        .background(RoundedRectangle(cornerRadius: 12).fill(Color.accent))
        .foregroundStyle(.white)
        .padding(.trailing, 16)
      VStack(alignment: .leading) {
        Text(item.markdown.title ?? item.frontMatter.title ?? "No title")
          .font(.headline)
        NoteDashboardBottomRow(
          item: item,
          searchBySubject: $searchBySubject,
          searchByTopic: $searchByTopic
        )
      }
      Spacer()
    }
    .contentShape(Rectangle())
    .padding()
    .glassEffect(in: .rect(cornerRadius: 12))
    .onTapGesture {
      appState.setEditingNote(editingNote: item)
      appState.mainView = defaultNoteView.toMainKey()
    }
  }
}

#Preview {
  NoteDashboardItem(
    item: NoteModel.fromNoteBody(noteBody: "# My note \n"), searchByTopic: .constant(nil),
    searchBySubject: .constant(nil))
}
