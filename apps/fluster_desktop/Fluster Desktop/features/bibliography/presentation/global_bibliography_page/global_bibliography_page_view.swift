//
//  global_bibliography_page_view.swift
//  Fluster
//
//  Created by Andrew on 1/18/26.
//

import FlusterData
import SwiftData
import SwiftUI

struct GlobalBibliographyPageView: View {
  @Query(sort: \BibEntryModel.title, order: .forward) var entries: [BibEntryModel]
  var body: some View {
    BibliographyEntryListView(entries: entries, abstractLineLimit: 20)
      .toolbar {
        ToolbarItem {
          Button(
            action: {
                AppState.shared.commandPaletteNavigate(to: .createBibEntry)
            },
            label: {
              Label(
                title: {
                  Text("Create")
                },
                icon: {
                  Image(systemName: "plus")
                })
            })
        }
      }
  }
}

#Preview {
  GlobalBibliographyPageView()
}
