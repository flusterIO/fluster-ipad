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
      List(paginatedSubjects) { subject in
        NavigationLink(
          destination: {
            Text("Here")
              .navigationTitle("Navigation Title Here")
          },
          label: {
            HStack {
              Circle()
                .fill(AutoTaggableType.subject.taggableColor())
                .frame(width: 6, height: 6)
              Text(subject.value)
            }
          })
      }
    }
    .padding()
    .glassEffect(in: .rect(cornerRadius: 12))
  }
}

#Preview {
  SubjectsDashboardSectionView()
}
