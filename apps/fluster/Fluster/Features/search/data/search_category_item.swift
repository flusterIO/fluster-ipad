//
//  search_category_item.swift
//  Fluster
//
//  Created by Andrew on 11/1/25.
//

enum SearchCategoryId {
  case note, citation, createNote
}

struct SearchCategoryItem: Identifiable {
  var id: SearchCategoryId
  var label: String
  var icon: String
}

let searchCategoryItems: [SearchCategoryItem] = [
  SearchCategoryItem(id: .note, label: "Note", icon: "text.page.fill"),
  SearchCategoryItem(id: .citation, label: "Citation", icon: "long.text.page.and.pencil.fill"),
  SearchCategoryItem(id: .createNote, label: "Create Note", icon: "plus")
]
