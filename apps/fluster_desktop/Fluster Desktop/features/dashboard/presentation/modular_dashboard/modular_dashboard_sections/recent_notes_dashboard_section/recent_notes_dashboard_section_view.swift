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
            NoteDashboardItem(item: note)
              .frame(maxWidth: .infinity)
          }
        }
      }
      .frame(minHeight: 400)
//      .glassEffect(in: .rect(cornerRadius: 12))
    }
    .padding()
    .glassEffect(in: .rect(cornerRadius: 12))
  }
}

#Preview {
  RecentNotesDashboardSectionView()
}
