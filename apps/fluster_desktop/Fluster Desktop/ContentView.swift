//
//  ContentView.swift
//  Fluster Desktop
//
//  Created by Andrew on 1/14/26.
//

import FlusterData
import SwiftData
import SwiftUI

struct ContentView: View {
  @AppStorage(DesktopAppStorageKeys.colorScheme.rawValue) private var selectedTheme: AppTheme =
    .dark
  @ObservedObject private var appState: AppState = AppState.shared
  @State private var columnVisibility: NavigationSplitViewVisibility = NavigationSplitViewVisibility
    .doubleColumn

  var body: some View {
    NavigationSplitView(columnVisibility: $columnVisibility) {
      MainViewSidebar()
    } detail: {
      NavigationStack(path: $appState.commandPaletteNavigation) {
        MainViewSwitch()
          .navigationDestination(
            for: CommandPaletteSecondaryView.self,
          ) { val in
            switch val {
              case .searchBySubject(let subject):
                SearchBySubjectView(item: subject)
              case .searchByTag(let tag):
                SearchByTagView(item: tag)
              case .searchByTopic(let topic):
                SearchByTopicView(item: topic)
              case .searchByCitation(let citation):
                SearchByCitationView(citation: citation)
              case .associateByEntriesWithEditingNote:
                AssociateNoteWithBibEntryView(editingNoteId: appState.editingNoteId)
              case .createBibEntry:
                BibtexEditorWebview(
                  editingNoteId: appState.editingNoteId, editingBibEntry: .constant(nil))
            }
          }
      }
    }
    .preferredColorScheme(selectedTheme.colorScheme)
    .environmentObject(appState)
    .onReceive(MainNavigationEventEmitter.shared.viewUpdatePublisher) { newValue in
      appState.mainView = newValue
    }
  }
}
