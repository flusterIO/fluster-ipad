//
//  sidebar_items.swift
//  fluster_desktop
//
//  Created by Andrew on 1/14/26.
//

import Foundation

struct SidebarItem: Codable, Identifiable {
    let label: String;
    let icon: String;
    let id: MainViewKey;
    let children: [SidebarItem]?
}


let mainSidebarItems: [SidebarItem] = [
    SidebarItem(label: "Dashboard", icon: "house.circle.fill", id: .dashboard, children: nil),
    SidebarItem(label: "Search", icon: "magnifyingglass.circle.fill", id: .search, children: nil),
    SidebarItem(label: "Settings", icon: "gearshape.fill", id: .settings, children: nil),
]

let noteSideBarItems: [SidebarItem] = [
    SidebarItem(label: "Create", icon: "plus", id: .createNote, children: nil),
    SidebarItem(label: "Markdown", icon: "text.document", id: .noteViewMdx, children: nil),
    SidebarItem(label: "Paper", icon: "scribble.variable", id: .paper, children: nil),
    SidebarItem(label: "Editor", icon: "keyboard", id: .noteEditingPage, children: nil),
]
