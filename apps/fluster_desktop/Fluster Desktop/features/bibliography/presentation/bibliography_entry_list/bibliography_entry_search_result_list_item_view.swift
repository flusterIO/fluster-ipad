//
//  bibliography_entry_search_result_list_item_view.swift
//  Fluster
//
//  Created by Andrew on 2/18/26.
//

import FlusterBibliography
import FlusterData
import SwiftUI

struct FormattedCitationData {
  let title: String?
  let url: String?
  let note: String?
  let abstract: String?
  let formatted: String?
}

struct BibliographyEntrySearchResultListItemView: View {
  let item: BibEntryModel
  @AppStorage(AppStorageKeys.embeddedCslFile.rawValue) private var cslFile: EmbeddedCslFileSwift =
    .apa

  var citationData: FormattedCitationData? {
    if let data = item.toBiblatexData() {
      FormattedCitationData(
        title: data.getTitle(), url: data.getUrl(), note: data.getNote(),
        abstract: data.getAbstract(),
        formatted: data.formatBibliographyCitation(
          cslContent: cslFile.toFlusterBibliographyCslFile(), cslLocale: getCslLocaleFileContent(),
          renderMethod: .plaintext))
    } else {
      nil
    }
  }

  var body: some View {
    if let data = citationData {
      VStack(alignment: .leading) {
        Text(data.formatted ?? data.title ?? "No title found")
          .font(.headline)
          .padding(.horizontal)
          .padding((data.note != nil || data.abstract != nil) ? .top : .vertical)
          .frame(maxWidth: .infinity, alignment: .leading)
        if let note = data.note {
          Text(note).font(.footnote)
            .lineLimit(3)
            .opacity(0.8)
            .padding(.horizontal)
            .padding(.bottom)
            .frame(maxWidth: .infinity, alignment: .leading)
        } else if let abstract = data.abstract {
          Text(abstract)
            .lineLimit(3)
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
        AppState.shared.commandPaletteNavigate(to: .searchByCitation(item))
      }
    } else {
      Color.clear
    }
  }
}

//#Preview {
//    BibliographyEntrySearchResultListItemView(item: BibEntryModel(
//        data: <#T##String#>
//    ))
//}
