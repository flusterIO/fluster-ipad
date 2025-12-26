//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/9/25.
//

import SwiftData
import SwiftUI

public struct TagSearchResultList: View {
  @Query(sort: \TagModel.lastAccess, order: .reverse) private var tags: [TagModel]
  @Binding var tagQuery: String
  @Binding var editingNote: NoteModel?
  public init(tagQuery: Binding<String>, editingNote: Binding<NoteModel?>) {
    self._tagQuery = tagQuery
    self._editingNote = editingNote
    if !tagQuery.wrappedValue.isEmpty {
      let s = tagQuery.wrappedValue  // Can't access this in the predicate directly.
      _tags = Query(
        filter: #Predicate<TagModel> { tag in
          tag.value.localizedStandardContains(
            s
          )
        },
        sort: [SortDescriptor(\TagModel.lastAccess, order: .reverse)],
        animation: .default
      )
    }
  }
  public var body: some View {
    List(tags) { tag in
      NavigationLink(
        destination: {
          NoteSearchResultsByTagView(tag: tag, editingNote: $editingNote)
        },
        label: {
          Text(tag.value)
        })
    }
    .searchable(text: $tagQuery, prompt: "Search Tags")
  }
}

#Preview {
  TagSearchResultList(tagQuery: .constant(""), editingNote: .constant(nil))
}
