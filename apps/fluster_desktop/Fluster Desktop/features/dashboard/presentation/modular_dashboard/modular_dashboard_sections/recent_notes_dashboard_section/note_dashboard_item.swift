//
//  note_dashboard_item.swift
//  Fluster
//
//  Created by Andrew on 1/19/26.
//

import FlusterData
import SwiftUI

struct NoteDashboardItem: View {
  let item: NoteModel
  var body: some View {
    HStack(alignment: .center) {
      Image(systemName: "text.document")
        .resizable()
        .frame(width: 24, height: 24)
        .padding(8)
        .background(RoundedRectangle(cornerRadius: 12).fill(Color.accent))
        .foregroundStyle(.white)
        .padding(.trailing, 16)
      VStack(alignment: .leading) {
        Text(item.markdown.title ?? item.frontMatter.title ?? "No title")
          .font(.headline)
        HStack(alignment: .center) {
          if let subject = item.subject {
            NavigationLink(
              destination: {
                Text("Search By Subject")
              },
              label: {
                Text(subject.value)
                  .font(.caption)
                  .padding(.horizontal, 4)
                  .padding(.vertical, 2)
                  .foregroundStyle(.white)
                  .background(
                    RoundedRectangle(cornerRadius: 12).fill(
                      AutoTaggableType.subject.taggableColor()))
                  .buttonStyle(.plain)
              })
          } else {
            Text("No Subject")
              .font(.caption)
              .foregroundStyle(.secondary)
          }
          Text("•")
            .font(.caption)
          if let topic = item.topic {
            NavigationLink(
              destination: {
                Text("Search By Topic")
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
              })
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
      Spacer()
    }
    .padding()
    .glassEffect(in: .rect(cornerRadius: 12))
    .onTapGesture {
        print("Here")
    }
  }
}

#Preview {
  NoteDashboardItem(item: NoteModel.fromNoteBody(noteBody: "# My note \n"))
}
