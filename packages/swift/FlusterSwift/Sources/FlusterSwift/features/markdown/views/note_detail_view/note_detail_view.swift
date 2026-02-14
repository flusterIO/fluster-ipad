//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/4/25.
//

import SwiftUI
import FlusterData

#if os(iOS)
public struct NoteDetailView: View {
  @Environment(ThemeManager.self) var themeManager: ThemeManager
  public let note: NoteModel
  public init(note: NoteModel) {
    self.note = note
  }
  var dateFormatter: RelativeDateTimeFormatter {
    let formatter = RelativeDateTimeFormatter()
    formatter.unitsStyle = .full
    return formatter
  }
  public var body: some View {
    VStack(alignment: .leading, spacing: 16) {
      Text("Title")
        .font(.title2)
        .foregroundStyle(themeManager.theme.muted_foreground)
      Text(note.markdown.title ?? "Unable to parse title")
        .font(.largeTitle)
        .padding(.horizontal, 24)
        .bold()
      Text(
        "Last modified \(dateFormatter.localizedString(for: note.utime, relativeTo: .now))"
      )
      .font(.subheadline)
      //            .padding(.horizontal, 8)
      .fontWeight(.light)
      .foregroundStyle(themeManager.theme.muted_foreground)
      Text(
        "Last read \(dateFormatter.localizedString(for: note.lastRead, relativeTo: .now))"
      )
      .font(.subheadline)
      .fontWeight(.light)
      //            .padding(.horizontal, 8)
      .foregroundStyle(themeManager.theme.muted_foreground)
      Divider()
      Text("Tags")
        .font(.title2)
        .foregroundStyle(themeManager.theme.muted_foreground)
      if note.tags.isEmpty {
        HStack(alignment: .center) {
          Text("No tags found")
            .font(.caption)
        }
      } else {
        HStack(
          alignment: .firstTextBaseline,
          content: {
            ForEach(note.tags, id: \.self) { tag in
              NavigationLink(
                destination: {
                  // FIXME: Replace this with the search by tag view that's yet to be created.
                  Text("Replace me with search by tag view.")
                },
                label: {
                  Text(tag.value)
                    .padding()
                    .background(Color(.systemGray6))
                    .cornerRadius(8)
                }
              )
            }
          }
        )
        .padding(.horizontal, 24)
      }
      Spacer()
    }
    .padding()
  }
}

#Preview {
  NoteDetailView(
    note: NoteModel.getEmptyModel(
      noteBody: "## Test Note",
      noteSummary:
        "This is my note summary here. This is my note summary here. This is my note summary here. This is my note summary here. This is my note summary here. This is my note summary here. This is my note summary here. This is my note summary here. This is my note summary here. This is my note summary here. This is my note summary here. "
    )
  )
}
#endif
