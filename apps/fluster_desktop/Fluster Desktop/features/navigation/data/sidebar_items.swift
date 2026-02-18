//
//  sidebar_items.swift
//  fluster_desktop
//
//  Created by Andrew on 1/14/26.
//

import FlusterData
import Foundation

struct SidebarItem: Codable, Identifiable {
  let label: String
  let icon: FlusterCategoryIcon
  let id: MainViewKey
  let children: [SidebarItem]?
}

let mainSidebarItems: [SidebarItem] = [
  SidebarItem(label: "Dashboard", icon: .dashboard, id: .dashboard, children: nil),
  SidebarItem(label: "Search", icon: .search, id: .search, children: nil),
  SidebarItem(label: "Settings", icon: .settings, id: .settings, children: nil)
]

let globalSearchSidebarItems: [SidebarItem] = [
  SidebarItem(label: "Bibliography", icon: .bibliography, id: .globalBibliography, children: nil)
]

let noteSideBarItems: [SidebarItem] = [
  SidebarItem(label: "Editor", icon: .editor, id: .noteEditingPage, children: nil),
  SidebarItem(label: "Paper", icon: .paper, id: .paper, children: nil),
  SidebarItem(label: "Markdown", icon: .markdown, id: .noteViewMdx, children: nil),
  SidebarItem(
    label: "Bibliography", icon: .bibliography, id: .editingNoteBibliography, children: nil),
  SidebarItem(
    label: "Details", icon: .noteDetails, id: .editingNoteDetails, children: nil),
  SidebarItem(label: "Create", icon: .create, id: .createNote, children: nil)
]
