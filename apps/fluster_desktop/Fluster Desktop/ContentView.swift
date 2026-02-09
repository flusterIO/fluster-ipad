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
  @Environment(\.modelContext) private var modelContext
  @EnvironmentObject private var appState: AppState
  var editingNote: NoteModel? {
    guard let id = appState.editingNoteId else { return nil }
    return modelContext.model(for: id) as? NoteModel
  }

  @Query private var items: [Item]
  @State private var columnVisibility: NavigationSplitViewVisibility = NavigationSplitViewVisibility
    .doubleColumn
  @State private var paths = NavigationPath()

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
            }
          }
      }
    }
    .onChange(
      of: editingNote?.markdown.body,
      {
        Task {
          if let note = editingNote {
            if let parsedMdx =
              await note.markdown
              .body.preParseAsMdxToBytes(noteId: note.id)
            {
              if let parsingResults =
                parsedMdx.toMdxParsingResult()
              {
                note.applyMdxParsingResults(
                  results: parsingResults,
                  modelContext: modelContext
                )
                Task {
                  do {
                    try modelContext.save()
                  } catch {
                    print(
                      "Failed to save model context when navigating away from editor view. Error: \(error.localizedDescription)"
                    )
                  }
                }
              }
            } else {
              print("Could not parse mdx.")
            }
          }
        }
      }
    )
    .onReceive(MainNavigationEventEmitter.shared.viewUpdatePublisher) { newValue in
      appState.mainView = newValue
    }
  }
}

#Preview {
  ContentView()
    .modelContainer(for: Item.self, inMemory: true)
}
