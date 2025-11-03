//
//  empty_bib_list_view.swift
//  Fluster
//
//  Created by Andrew on 11/3/25.
//

import SwiftUI

struct EmptyBibListView: View {
    @Environment(ThemeManager.self) private var themeManager: ThemeManager
    
    @State private var createSheetOpen = false

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
            Text("No bibliography entries found")
                .font(.title2)
            Button("Create") {
                createSheetOpen = true
            }
        }
        .sheet(isPresented: $createSheetOpen, content: {
            CreateBibEntrySheetView(isPresented: $createSheetOpen)
        })
    }
}

#Preview {
    EmptyBibListView()
}
