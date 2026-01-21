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
  @Environment(AppState.self) private var appState: AppState
  @Query private var items: [Item]

  var body: some View {
    NavigationSplitView {
      MainViewSidebar()
    } detail: {
      NavigationStack {
        ZStack {
          MainViewSwitch()
          CommandPaletteContainerView()
        }
      }
    }
    .onChange(
        of: appState.editingNote?.markdown.body,
      {
        Task {
            if let note = appState.editingNote {
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
              }
              Task {
                do {
                  try modelContext.save()
                } catch {
                  print(
                    "Failed to save model context when navigating away from editor view."
                  )
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
