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

// 1. PIN THE TYPEALIAS TO THE MAIN ACTOR
// This ensures that the closure body is always isolated to the main thread.
public typealias EvalJavascriptFunc = @MainActor @Sendable (String) async throws -> Sendable?

public struct GenerateAINoteSummaryButton: View {
  @Environment(\.colorScheme) private var colorScheme: ColorScheme
  @AppStorage(AppStorageKeys.codeBlockThemeDark.rawValue) var codeBlockThemeDark:
    SupportedCodeBlockTheme = .solarizedDark
  @AppStorage(AppStorageKeys.codeBlockThemeLight.rawValue) var codeBlockThemeLight:
    SupportedCodeBlockTheme = .solarizedLight
  public var editingNote: NoteModel?
  public var evalJs: EvalJavascriptFunc

  @AppStorage(AppStorageKeys.userPreferredName.rawValue)
  private var userPreferredName: String?

  @State private var summary: String.PartiallyGenerated? = nil

  public init(
    editingNote: NoteModel?,
    evalJavascript: @escaping EvalJavascriptFunc
  ) {
    self.editingNote = editingNote
    self.evalJs = evalJavascript
  }

  public var body: some View {
    Button(action: {
      performSummaryGeneration()
    }) {
      Label(
        title: { Text("Summarize") },
        icon: { Image(systemName: "pencil") }
      )
    }
  }

  private func performSummaryGeneration() {
    guard let note = editingNote else { return }

    let noteId = note.id
    let preferredName = userPreferredName

    // Start background processing
    Task {
      do {
        let session = getNoteSummaryLanguageModelSession(
          AIUserDetails(preferred_name: preferredName),
        )
        let uiParams = UiParams(
          darkMode: colorScheme == .dark, fontScalar: 1, mathFontScalar: 1.2,
          syntaxTheme: .solarizedDark)

        let inputContent = try await note.markdown.body.conundrumToAIInput(
          noteId: noteId,
          uiParams: uiParams)
        let responseStream = try await session.streamResponse(to: Prompt(inputContent))

        var isInitial = true

        for try await partial in responseStream {
          // Keep heavy work on the background thread
          let options = ParseConundrumOptions(
            noteId: noteId,
            content: partial.content,
            modifiers: [],
            hideComponents: [],
            uiParams: uiParams,
            target: .markdown,
            trusted: true
          )

          let parsed = try await ConundrumSwift.runConundrum(options: options)
          let serializedString = parsed.content.toFlatBufferSerializedString()

          let isFirstChunk = isInitial
          isInitial = false

          // Construct the JS string on the background thread
          let js = "window.sendNoteSummaryStream(\(serializedString), \(isFirstChunk))"

          // 2. DISPATCH TO MAIN ACTOR
          // Because 'evalJs' is now @MainActor, this call is guaranteed safe.
          Task { @MainActor in
            // Update UI state
            self.summary = partial.content

            do {
              // This call will no longer trigger a Main Thread warning
              _ = try await evalJs(js)
            } catch {
              print("JS Stream Error: \(error)")
            }
          }
        }
      } catch {
        print("Generation failed: \(error.localizedDescription)")
      }
    }
  }
}
