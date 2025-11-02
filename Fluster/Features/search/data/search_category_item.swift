//
//  search_category_item.swift
//  Fluster
//
//  Created by Andrew on 11/1/25.
//

enum SearchCategory {
    case note, citation
}

struct SearchCategoryItem: Identifiable {
    var id: SearchCategory
    var label: String
    var icon: String
}


let searchCategoryItems: [SearchCategoryItem] = [
    SearchCategoryItem(id: .note, label: "Note", icon: "book"),
    SearchCategoryItem(id: .citation, label: "Citation", icon: "book")
]
