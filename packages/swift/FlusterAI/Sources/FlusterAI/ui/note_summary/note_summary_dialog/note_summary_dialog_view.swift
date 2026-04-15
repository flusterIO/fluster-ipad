//
//  File.swift
//  FlusterAI
//
//  Created by Andrew on 4/14/26.
//

import FoundationModels
import SwiftUI

struct MarkdownTextViewCopy: View {
  let content: String
  let syntax: AttributedString.MarkdownParsingOptions.InterpretedSyntax
  private var attributedContent: AttributedString {
    do {
      return try AttributedString(
        markdown: content,
        options: .init(interpretedSyntax: syntax)
      )
    } catch {
      return AttributedString(content)
    }
  }
  init(
    _ content: String, _ syntax: AttributedString.MarkdownParsingOptions.InterpretedSyntax
  ) {
    self.content = content
    self.syntax = syntax
  }
  var body: some View {
    Text(attributedContent)
  }
}

public struct NoteSummaryDialogView: View {
  private var summary: String.PartiallyGenerated
  public init(summary: String.PartiallyGenerated) {
    self.summary = summary
  }
  public var body: some View {
    VStack {
      Text("Summary")
        .font(.title)
        .alignmentGuide(
          .leading,
          computeValue: { vd in
            vd.width
          }
        )
        .bold()
      ScrollView {
        MarkdownTextViewCopy(summary, .full)
          .foregroundStyle(.secondary)
      }
      //        .frame(minHeight: 300)
      // .frame(height: min(geo.size.)
      HStack(alignment: .center) {
        Spacer()
        Button(
          action: {
          },
          label: {
            Label(
              title: {
                Text("Retry")
              },
              icon: {
                Image(systemName: "checkmark.arrow.trianglehead.counterclockwise")
              })
          })
        Button(
          action: {
          },
          label: {
            Label(
              title: {
                Text("Accept")
              },
              icon: {
                Image(systemName: "checkmark")
              })
          })
      }
    }
    .padding()
    .clipShape(RoundedRectangle(cornerRadius: 12, style: .continuous))
  }
}

#Preview {
  NoteSummaryDialogView(summary: "Lorem ipsum mfer.")
}
