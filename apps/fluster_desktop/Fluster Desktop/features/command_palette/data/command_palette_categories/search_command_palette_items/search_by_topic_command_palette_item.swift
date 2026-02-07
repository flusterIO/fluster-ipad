//
//  search_by_topic_command_palette_item.swift
//  Fluster
//
//  Created by Andrew on 1/20/26.
//

import Foundation
import FlusterData
import SwiftData

class SearchByTopicCommandPaletteItem: CommandPaletteItem {
  init() {
    super.init(
      id: .parentWithNoFunctionality, uniqueId: "searcyByTopic", title: "Search by Topic", icon: "tag.fill", subtitle: nil,
      itemType: .children, noneFoundText: "No topics found.")
  }
    public override func children(modelContext: ModelContext, appState: AppState) -> [CommandPaletteItem] {
    let fetchDescriptor = FetchDescriptor<TopicModel>(
      sortBy: [
        SortDescriptor(\TopicModel.lastAccess, order: .reverse)
      ],
    )
    do {
      let topics = try modelContext.fetch(fetchDescriptor)
      return topics.map { topic in
        CommandPaletteItem(
          id: .pushCommandPaletteView(.searchByTopic(topic)),
          uniqueId: "topic-\(topic.value)",
          title: topic.value, icon: "tag.fill",
          subtitle: nil,
          itemType: .commandPaletteAction)
      }
    } catch {
      print("Error retrieving tags: \(error.localizedDescription)")
    }
    return []
  }
}
