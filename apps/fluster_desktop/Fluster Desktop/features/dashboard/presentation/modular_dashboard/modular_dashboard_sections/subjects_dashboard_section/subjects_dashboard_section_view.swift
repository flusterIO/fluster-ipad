//
//  subjects_dashboard_section_view.swift
//  Fluster
//
//  Created by Andrew on 1/19/26.
//

import FlusterData
import SwiftData
import SwiftUI

struct SubjectsDashboardSectionView: View {
  @Query(sort: \SubjectModel.lastAccess, order: .reverse) var subjects: [SubjectModel]
  @State private var pageIdx: Int = 0
  let perPage: Int = 5
  var paginatedSubjects: ArraySlice<SubjectModel> {
    subjects[(pageIdx * perPage)..<min(((pageIdx + 1) * perPage), subjects.count)]
  }
  var body: some View {
    VStack(alignment: .leading) {
      Text("Subjects")
        .font(.title2)
      Text("Click an item to search by subject.")
        .font(.caption)
        .foregroundStyle(.secondary)
      List(paginatedSubjects) { subject in
        NavigationLink(
          destination: {
            SearchBySubjectView(item: subject)
              .navigationTitle("Notes found by subject")
          },
          label: {
            HStack {
              Circle()
                .fill(AutoTaggableType.subject.taggableColor())
                .frame(width: 6, height: 6)
              Text(subject.value)
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
      .scrollContentBackground(.hidden)
    }
    .padding()
    .glassEffect(in: .rect(cornerRadius: 12))
  }
}

#Preview {
  SubjectsDashboardSectionView()
}
