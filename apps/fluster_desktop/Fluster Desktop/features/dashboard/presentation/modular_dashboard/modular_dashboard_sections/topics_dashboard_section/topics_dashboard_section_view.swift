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
      List(paginatedTopics) { topic in
        NavigationLink(
          destination: {
            Text("Here")
          },
          label: {
            HStack {
              Circle()
                .fill(AutoTaggableType.topic.taggableColor())
                .frame(width: 6, height: 6)
              Text(topic.value)
            }
          })
      }
    }
    .padding()
    .glassEffect(in: .rect(cornerRadius: 12))
  }
}

#Preview {
  TopicsDashboardSectionView()
}
