//
//  quick_actions.swift
//  Fluster
//
//  Created by Andrew on 1/18/26.
//

import SwiftUI
import FlusterData

public struct QuickAction {
    let label: String
    let icon: String
    let color: Color
    let action: (AppState) -> Void
}


public let quickActions: [QuickAction] = [
    QuickAction(label: "Bookmarks", icon: FlusterCategoryIcon.bookmarks.toSfIcon(), color: Color.indigo, action: { appState in
        appState.mainView = .bookmarks
    }),
    QuickAction(label: "Conundrum", icon: "text.document", color: Color.orange, action: { appState in
        appState.mainView = .noteViewMdx
    }),
    QuickAction(label: "Editor", icon: "pencil", color: Color.green, action: { appState in
        appState.mainView = .noteEditingPage
    }),
    QuickAction(label: "Bibliography", icon: FlusterCategoryIcon.bibliography.toSfIcon(), color: Color.purple, action: { appState in
        appState.mainView = .globalBibliography
    }),
    QuickAction(label: "Create Note", icon: "plus.rectangle.on.folder.fill", color: Color.cyan, action: { appState in
        appState.mainView = .createNote
    }),
    QuickAction(label: "Search", icon: "text.page.badge.magnifyingglass", color: Color.pink, action: { appState in
        appState.mainView = .search
    }),
]
