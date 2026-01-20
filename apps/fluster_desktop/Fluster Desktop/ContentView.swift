//
//  ContentView.swift
//  Fluster Desktop
//
//  Created by Andrew on 1/14/26.
//

import SwiftData
import SwiftUI
import FlusterData

struct ContentView: View {
  @Environment(\.modelContext) private var modelContext
  @Environment(AppState.self) private var appState: AppState
  @Query private var items: [Item]

  var body: some View {
    NavigationSplitView {
      MainViewSidebar()
    } detail: {
      ZStack {
        NavigationStack {
          MainViewSwitch()
        }
        CommandPaletteContainerView()
      }
    }
    .onReceive(MainNavigationEventEmitter.shared.viewUpdatePublisher) { newValue in
      appState.mainView = newValue
    }
  }
}

#Preview {
  ContentView()
    .modelContainer(for: Item.self, inMemory: true)
}
