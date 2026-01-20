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
    .onReceive(MainNavigationEventEmitter.shared.viewUpdatePublisher) { newValue in
      appState.mainView = newValue
    }
  }
}

#Preview {
  ContentView()
    .modelContainer(for: Item.self, inMemory: true)
}
