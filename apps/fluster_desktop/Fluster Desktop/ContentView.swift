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
  var editingNoteId: String?
  @Environment(\.modelContext) private var modelContext
  @EnvironmentObject private var appState: AppState
  @Query private var notes: [NoteModel]

  var editingNote: NoteModel? {
    notes.isEmpty ? nil : notes.first!
  }

  @Query private var items: [Item]
  @State private var columnVisibility: NavigationSplitViewVisibility = NavigationSplitViewVisibility
    .doubleColumn
  @State private var paths = NavigationPath()

  public init(editingNoteId: String?) {
    self.editingNoteId = editingNoteId
    if let _id = editingNoteId {
      let predicate = #Predicate<NoteModel> { $0.id == _id }
      _notes = Query(filter: predicate)
    } else {
      _notes = Query(
        filter: #Predicate<NoteModel> { note in
          false
        })
    }
  }

  var body: some View {
    NavigationSplitView(columnVisibility: $columnVisibility) {
      MainViewSidebar()
    } detail: {
      NavigationStack(path: $appState.commandPaletteNavigation) {
        MainViewSwitch(editingNoteId: appState.editingNoteId)
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
          if editingNote == nil || editingNote?.isDeleted == true {
            return
          }
          if let note = editingNote, editingNoteIsValid(note: note, appState: appState) {
            if let parsedMdx =
              await note.markdown.body.preParseAsMdxToBytes(noteId: note.id)
            {
              if let parsingResults =
                parsedMdx.toMdxParsingResult()
              {
                note.applyMdxParsingResults(
                  results: parsingResults,
                  modelContext: modelContext
                )
              }
            } else {
              print("Could not parse mdx.")
            }
          }
        }
        modelContext.saveChanges()
      }
    )
    .onReceive(MainNavigationEventEmitter.shared.viewUpdatePublisher) { newValue in
      appState.mainView = newValue
    }
  }
}

#Preview {
  ContentView(editingNoteId: nil)
    .modelContainer(for: Item.self, inMemory: true)
}
