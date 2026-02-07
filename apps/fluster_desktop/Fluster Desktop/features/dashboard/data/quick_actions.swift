//
//  quick_actions.swift
//  Fluster
//
//  Created by Andrew on 1/18/26.
//

import SwiftUI

public struct QuickAction {
    let label: String
    let icon: String
    let color: Color
    let action: (AppState) -> Void
}


public let quickActions: [QuickAction] = [
    QuickAction(label: "Bookmarks", icon: "bookmark.fill", color: Color.indigo, action: { appState in
        appState.mainView = .bookmarks
    }),
    QuickAction(label: "Equations", icon: "function", color: Color.orange, action: { appState in
        appState.mainView = .dashboard
    }),
    QuickAction(label: "Snippets", icon: "keyboard.fill", color: Color.green, action: { appState in
        appState.mainView = .dashboard
    }),
    QuickAction(label: "Bibliography", icon: "book.pages.fill", color: Color.purple, action: { appState in
        appState.mainView = .globalBibliography
    }),
    QuickAction(label: "Create Note", icon: "plus.rectangle.on.folder.fill", color: Color.cyan, action: { appState in
        appState.mainView = .createNote
    }),
    QuickAction(label: "Search", icon: "magnifyingglass", color: Color.pink, action: { appState in
        appState.mainView = .search
    }),
]
