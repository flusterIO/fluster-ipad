//
//  note_dashboard_item_bottom_row.swift
//  Fluster
//
//  Created by Andrew on 1/19/26.
//

import FlusterData
import SwiftUI

struct NoteDashboardBottomRow: View {
  let item: NoteModel
  @Binding var searchBySubject: SubjectModel?
  @Binding var searchByTopic: TopicModel?
  var body: some View {
    HStack(alignment: .center) {
      if let subject = item.subject {
        Button(
          action: {
              searchBySubject = subject
          },
          label: {
            Text(subject.value)
              .font(.caption)
              .padding(.horizontal, 4)
              .padding(.vertical, 2)
              .foregroundStyle(.white)
              .background(
                RoundedRectangle(cornerRadius: 12).fill(
                  AutoTaggableType.subject.taggableColor()
                )
              )
              .buttonStyle(.plain)
          }
        )
        .buttonStyle(.plain)
      } else {
        Text("No Subject")
          .font(.caption)
          .foregroundStyle(.secondary)
      }
      Text("•")
        .font(.caption)
      if let topic = item.topic {
        Button(
          action: {
              searchByTopic = topic
          },
          label: {
            Text(topic.value)
              .font(.caption)
              .padding(.horizontal, 4)
              .padding(.vertical, 2)
              .foregroundStyle(.white)
              .background(
                RoundedRectangle(cornerRadius: 12).fill(
                  AutoTaggableType.topic.taggableColor()))
          }
        )
        .buttonStyle(.plain)
      } else {
        Text("No Topic")
          .font(.caption)
          .foregroundStyle(.secondary)
      }
      Text("•")
        .font(.caption)
      Text(item.lastRead.formatted(date: .abbreviated, time: .shortened))
        .font(.caption)
        .foregroundStyle(.secondary)
    }
  }
}
