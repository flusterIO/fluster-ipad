//
//  note_search_result_item_view.swift
//  Fluster
//
//  Created by Andrew on 1/19/26.
//

import FlusterData
import SwiftUI

struct NoteSearchResultItemView: View {
  let item: NoteModel
  var body: some View {
    VStack(alignment: .leading) {
      Text(item.frontMatter.title ?? item.markdown.title ?? "No Title Found")
        .font(.title2)
      if item.markdown.summary != nil && !item.markdown.summary!.isEmpty {
        Text(item.markdown.summary!)
          .font(.subheadline)
          .foregroundStyle(.secondary)
      }
      NoteDashboardBottomRow(item: item)
    }
    .padding()
    .frame(maxWidth: .infinity, alignment: .leading)
    .glassEffect(in: .rect(cornerRadius: 12))
    .contentShape(Rectangle())
    .onTapGesture {
        print("Here")
    }
  }
}

#Preview {
  NoteSearchResultItemView(
    item: NoteModel.fromNoteBody(noteBody: "# My Note")
  )
}
