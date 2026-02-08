//
//  topics_dashboard_section_view.swift
//  Fluster
//
//  Created by Andrew on 1/19/26.
//

import FlusterData
import SwiftData
import SwiftUI

struct TopicsDashboardSectionView: View {
  @Query(sort: \TopicModel.lastAccess, order: .reverse) private var topics: [TopicModel]
  @State private var pageIdx: Int = 0
  let perPage: Int = 5
  var paginatedTopics: ArraySlice<TopicModel> {
    topics[(pageIdx * perPage)..<min(((pageIdx + 1) * perPage), topics.count)]
  }
  var body: some View {
    VStack(alignment: .leading) {
      Text("Topics")
        .font(.title2)
      Text("Click an item to search by topic.")
        .font(.caption)
        .foregroundStyle(.secondary)
      List(paginatedTopics) { topic in
        NavigationLink(
          destination: {
              SearchByTopicView(item: topic)
                  .navigationTitle("Notes found by topic")
          },
          label: {
            HStack(alignment: .center) {
              Circle()
                .fill(AutoTaggableType.topic.taggableColor())
                .frame(width: 6, height: 6)
              Text(topic.value)
            }
            .padding(.horizontal)
            .padding(.vertical, 8)
            .frame(maxWidth: .infinity, alignment: .leading)
            .glassEffect(in: .rect(cornerRadius: 12))
          }
        )
        .listRowBackground(Color.clear)
        .listRowSeparator(.hidden)
        .listRowSeparatorTint(Color.clear)
      }
      .frame(minHeight: 200)
      .scrollContentBackground(.hidden)
    }
    .padding()
    .glassEffect(in: .rect(cornerRadius: 12))
  }
}

#Preview {
  TopicsDashboardSectionView()
}
