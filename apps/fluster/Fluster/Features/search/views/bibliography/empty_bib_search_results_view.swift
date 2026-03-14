//
//  empty_bib_search_results_view.swift
//  Fluster
//
//  Created by Andrew on 11/5/25.
//

import FlusterData
import FlusterSwift
import FlusterWebviewClients
import SwiftUI

struct EmptyBibSearchResultsView: View {
  @Environment(ThemeManager.self) private var themeManager: ThemeManager
  @Binding private var errorMessage: EditorErrorMessage?

  init(errorMessage: Binding<EditorErrorMessage?>) {
    self._errorMessage = errorMessage
  }

  var body: some View {
    VStack(spacing: 16) {
      ZStack {
        themeManager.theme.primary
          .frame(width: 64, height: 64)
          .clipShape(Circle())
        Image(systemName: "magnifyingglass")
          .imageScale(.large)
          .foregroundStyle(themeManager.theme.primary_foreground)
      }
      Text("No entries found")
        .font(.title2)
      NavigationLink(
        destination: CreateBibEntrySheetView(
          ignoreEditingNote: true, editingNote: .constant(nil), editingBibEntry: .constant(nil),
          errorMessage: $errorMessage
        ),
        label: {
          Label("Create", systemImage: "plus")
        }
      )
      .buttonStyle(.borderedProminent)
    }
  }
}
