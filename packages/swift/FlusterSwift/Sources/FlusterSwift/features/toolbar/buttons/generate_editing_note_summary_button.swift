//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 4/13/26.
//

import ConundrumSwift
import FlusterAI
import FlusterData
import FoundationModels
import SwiftUI

public typealias EvalJavascriptFunc = @Sendable (String) async throws -> Sendable?

public struct GenerateAINoteSummaryButton: View {
  public var editingNote: NoteModel?
  public var evalJs: EvalJavascriptFunc
  @AppStorage(AppStorageKeys.userPreferredName.rawValue) private var userPreferredName: String?
  @State private var summary: String.PartiallyGenerated? = nil
  public init(
    editingNote: NoteModel?,
      evalJavascript: @escaping EvalJavascriptFunc
  ) {
    self.editingNote = editingNote
    self.evalJs = evalJavascript
  }
  public var body: some View {
    var summaryShowing: Binding<Bool> {
      Binding(
        get: {
          self.summary != nil
        },
        set: { newShowing in
          if !newShowing {
            self.summary = nil
          }
        })
    }
    Button(
      action: {
        Task {
          do {
            if let en = editingNote {
              let session = getNoteSummaryLanguageModelSession(
                AIUserDetails(preferred_name: userPreferredName))
              let content = try await en.markdown.body.conundrumToAIInput(
                noteId: en.id)
              let res = try await session.streamResponse(to: Prompt(content))
              var isInitial = true
              for try await partiallyGeneratedSummary in res {
                self.summary = partiallyGeneratedSummary.content
                let parsedContent = try await ConundrumSwift.runConundrum(
                  options: ParseConundrumOptions(
                    noteId: editingNote?.id, content: partiallyGeneratedSummary.content,
                    modifiers: [.preferMarkdownSyntax], hideComponents: []))
                let serializedString = parsedContent.content.toFlatBufferSerializedString()
                  // WITH_WIFI: Figure out how to run this on the main thread.
//                await MainActor.run(body: {
//                  try await evalJs(
//                    """
//                    window.sendNoteSummaryStream(\(serializedString), \(isInitial ? "true" : "false")
//                    """)
//                })
                isInitial = false
              }
            } else {
              // Show user notification here.
            }
          } catch {
            print("Error: \(error.localizedDescription)")
          }
        }
      },
      label: {
        Label(
          title: {
            Text("Summarize")
          },
          icon: {
            Image(systemName: "pencil")
          })
      }
    )
  }
}
