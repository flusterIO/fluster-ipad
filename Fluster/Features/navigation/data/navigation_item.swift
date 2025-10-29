//
//  navigation_item.swift
//  Fluster
//
//  Created by Andrew on 10/29/25.
//

import SwiftUI

struct NavigationItem: Identifiable {
    let id = UUID()
    var label: String
    var path: MainCoordinatorPages
    var icon: String
}

var navigationItems: [NavigationItem] = [
    NavigationItem(label: "Home", path: .root, icon: "house.fill"),
    NavigationItem(label: "Bibliography", path: .bibliography, icon: "book.pages.fill"),
    NavigationItem(label: "Search", path: .search, icon: "magnifyingglass.circle.fill"),
]
