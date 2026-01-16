//
//  ContentView.swift
//  Fluster Desktop
//
//  Created by Andrew on 1/14/26.
//

import SwiftData
import SwiftUI

struct ContentView: View {
  @Environment(\.modelContext) private var modelContext
  @Query private var items: [Item]
  @State private var mainView: MainViewKey = .dashboard

  var body: some View {
    NavigationSplitView {
      MainViewSidebar(mainView: $mainView)
    } detail: {
      ZStack {
        MainViewSwitch(mainView: $mainView)
        CommandPaletteContainerView()
      }
    }
    .onReceive(MainNavigationEventEmitter.shared.viewUpdatePublisher) { newValue in
      mainView = newValue
    }
  }
}

#Preview {
  ContentView()
    .modelContainer(for: Item.self, inMemory: true)
}
