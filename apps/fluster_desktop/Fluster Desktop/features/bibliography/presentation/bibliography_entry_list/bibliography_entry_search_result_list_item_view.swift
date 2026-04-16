//
//  bibliography_entry_search_result_list_item_view.swift
//  Fluster
//
//  Created by Andrew on 2/18/26.
//

import FlusterBibliography
import FlusterData
import SwiftUI

struct BibliographyEntrySearchResultListItemView: View {
  let item: BibEntryModel
  let abstractLineLimit: Int
  @AppStorage(AppStorageKeys.embeddedCslFile.rawValue) private var cslFile: EmbeddedCslFileSwift =
    .apa
  let toCitation: (BibEntryModel) -> Void

  var citationData: FormattedCitation? {
    item.safelyGetFormatted(activeCslFormat: cslFile)
  }

  var body: some View {
    if let data = citationData {
      VStack(alignment: .leading) {
        Text(data.formattedPlainText)
          .font(.headline)
          .padding(.horizontal)
          .padding((data.note != nil || data.abstract != nil) ? .top : .vertical)
          .frame(maxWidth: .infinity, alignment: .leading)
        if let note = data.note {
          Text(note).font(.footnote)
            .lineLimit(abstractLineLimit)
            .opacity(0.8)
            .padding(.horizontal)
            .padding(.bottom)
            .frame(maxWidth: .infinity, alignment: .leading)
        } else if let abstract = data.abstract {
          Text(abstract)
            .lineLimit(abstractLineLimit)
            .font(.footnote)
            .opacity(0.8)
            .padding(.horizontal)
            .padding(.bottom)
            .frame(maxWidth: .infinity, alignment: .leading)
        }
      }
      .frame(maxWidth: .infinity)
      .glassEffect(in: .rect(cornerRadius: 16))
      .onTapGesture {
        self.toCitation(item)
      }
    } else {
      Color.clear
    }
  }
}
