//
//  search_by_subject_command_palette_item.swift
//  Fluster
//
//  Created by Andrew on 1/20/26.
//

import FlusterData
import SwiftData
import SwiftUI

class SearchBySubjectCommandPaletteItem: CommandPaletteItem {
  init() {
    super.init(
      id: .parentWithNoFunctionality, title: "Search by Subject", icon: "tag.fill", subtitle: nil,
      itemType: .children, noneFoundText: "No subjects found.")
  }
  public override func children(modelContext: ModelContext, appState: AppState)
    -> [CommandPaletteItem]
  {
    let fetchDescriptor = FetchDescriptor<SubjectModel>(
      sortBy: [
        SortDescriptor(\SubjectModel.lastAccess, order: .reverse)
      ],
    )
    do {
      let subjects = try modelContext.fetch(fetchDescriptor)
      return subjects.map { subject in
        CommandPaletteItem(
          id: .pushCommandPaletteView(.searchBySubject(subject)), title: subject.value,
          icon: "tag.fill",
          subtitle: nil,
          itemType: .commandPaletteAction)
      }
    } catch {
      print("Error retrieving tags: \(error.localizedDescription)")
    }
    return []
  }
}
