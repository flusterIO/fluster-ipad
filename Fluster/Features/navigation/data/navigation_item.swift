//
//  navigation_item.swift
//  Fluster
//
//  Created by Andrew on 10/29/25.
//

import SwiftUI

struct NavigationItem: Identifiable {
    let id: String
    var label: String
    var path: MainCoordinatorPages
    var icon: String
}

var navigationItems: [NavigationItem] = [
    NavigationItem(id: "home", label: "Home", path: .root, icon: "house.fill"),
    NavigationItem(id: "bib", label: "Bibliography", path: .bibliography, icon: "book.pages.fill"),
    NavigationItem(id: "search", label: "Search", path: .search, icon: "magnifyingglass.circle.fill"),
    NavigationItem(id: "sketch", label: "Write", path: .paper, icon: "pencil.circle.fill"),
]
