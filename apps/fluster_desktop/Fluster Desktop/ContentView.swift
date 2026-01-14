//
//  ContentView.swift
//  Fluster Desktop
//
//  Created by Andrew on 1/14/26.
//

import SwiftUI
import SwiftData

struct ContentView: View {
    @Environment(\.modelContext) private var modelContext
    @Query private var items: [Item]
    @State private var mainView: MainViewKey = .dashboard

    var body: some View {
        NavigationSplitView {
            MainViewSidebar(mainView: $mainView)
        } detail: {
            MainViewSwitch(mainView: $mainView)
        }
    }

    private func addItem() {
        withAnimation {
            let newItem = Item(timestamp: Date())
            modelContext.insert(newItem)
        }
    }

    private func deleteItems(offsets: IndexSet) {
        withAnimation {
            for index in offsets {
                modelContext.delete(items[index])
            }
        }
    }
}

#Preview {
    ContentView()
        .modelContainer(for: Item.self, inMemory: true)
}
