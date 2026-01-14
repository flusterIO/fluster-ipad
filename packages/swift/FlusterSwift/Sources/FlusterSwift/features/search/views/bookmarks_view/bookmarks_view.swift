//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/11/25.
//

import SwiftData
import SwiftUI
import FlusterData

public struct BookmarksView: View {
  @Query(
    filter: #Predicate<NoteModel> { note in
      note.bookmarked
    },
    sort: \.lastRead,
    animation: .default
  ) private var notes: [NoteModel]
  @Binding var editingNote: NoteModel?
  public init(editingNote: Binding<NoteModel?>) {
    self._editingNote = editingNote
  }
  public var body: some View {
    if notes.isEmpty {
      NoNotesFoundView(
        title: "No notes found",
        subtitle:
          "No bookmarked notes were found. Use the floating button when a note is selected to bookmark that note."
      )
      .padding()
    } else {
      List(notes, id: \.id) { note in
        NoteSearchResultItemInnerView(editingNote: $editingNote, item: note)
          .swipeActions(content: {
            Button(
              role: .cancel,
              action: {
                withAnimation {
                  note.bookmarked.toggle()
                }
              },
              label: {
                Label("Remove Bookmark", systemImage: "bookmark.slash")
              })
          })
      }
    }
  }
}

#Preview {
  BookmarksView(editingNote: .constant(nil))
}
