//
//  recent_notes_dashboard_section_view.swift
//  Fluster
//
//  Created by Andrew on 1/19/26.
//

import FlusterData
import SwiftData
import SwiftUI

struct RecentNotesDashboardSectionView: View {
  @Query(sort: \NoteModel.lastRead, order: .reverse) private var notes: [NoteModel]
    @State private var searchByTopic: TopicModel? = nil
    @State private var searchBySubject: SubjectModel? = nil
  var body: some View {
    VStack(alignment: .leading) {
      VStack(alignment: .leading) {
        HStack {
          Image(systemName: "document.badge.clock")
            .font(.title)
          Text("Recent Notes")
            .font(.title)
        }
        Text("Your latest and most recently accessed notes")
          .font(.caption)
          .foregroundStyle(.secondary)
      }
      VStack(alignment: .center) {
        if notes.isEmpty {
          Text("No notes found")
            .font(.headline)
        } else {
          List(notes) { note in
            NoteDashboardItem(item: note, searchByTopic: $searchByTopic, searchBySubject: $searchBySubject)
              .frame(maxWidth: .infinity)
              .listRowBackground(Color.clear)
              .listRowSeparator(.hidden)
              .listRowSeparatorTint(Color.clear)
          }
          .scrollContentBackground(.hidden)
          .listRowBackground(Color.clear)
          .listRowSeparator(.hidden)
          .listRowSeparatorTint(Color.clear)
        }
      }
      .frame(minHeight: 400)
    }
    .padding()
    .glassEffect(in: .rect(cornerRadius: 12))
    .navigationDestination(item: $searchByTopic, destination: { topic in
        SearchByTopicView(item: topic)
    })
    .navigationDestination(item: $searchBySubject, destination: { subject in
        SearchBySubjectView(item: subject)
   })

  }
}

#Preview {
  RecentNotesDashboardSectionView()
}
