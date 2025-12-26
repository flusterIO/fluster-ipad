//
//  bib_entry_search_result_item_view.swift
//  Fluster
//
//  Created by Andrew on 11/9/25.
//

import FlusterSwift
import SwiftUI
import SwiftyBibtex

struct BibEntrySearchResultItemView: View {
  var item: BibEntryModel
  var body: some View {
    VStack(alignment: .leading, spacing: 16) {
      Text(item.parse()?.publications[0].fields["title"] ?? "--")
        .font(.subheadline)
        .fontWeight(.bold)
        .lineLimit(2)
      Text(
        item.ctime.formatted(date: .complete, time: .shortened)
      )
      .font(.caption)
    }
  }
}

#Preview {
  BibEntrySearchResultItemView(
    item: BibEntryModel(
      id: nil,
      data: """
        @book{latex2e,
          author = {Leslie Lamport},
          year = {1994},
          title = {{\\LaTeX}: a Document Preparation System},
          publisher = {Addison Wesley},
          address = {Massachusetts},
          edition = {2}
        }
        """,
      ctime: .now
    ),
  )
}
