//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 4/13/26.
//

import FlusterAI
import FlusterData
import FoundationModels
import SwiftUI

public struct GenerateAINoteSummaryButton: View {
  public var editingNote: NoteModel?
  @AppStorage(AppStorageKeys.userPreferredName.rawValue) private var userPreferredName: String?
  public init(editingNote: NoteModel?) {
    self.editingNote = editingNote
  }
  public var body: some View {
    Button(
      action: {
        Task {
          do {
            if let en = editingNote {
              let session = getNoteSummaryLanguageModelSession(
                AIUserDetails(preferred_name: userPreferredName))
              let content = try await en.markdown.body.conundrumToAIInput(
                noteId: en.id)
              print("AI Input: \(content)")
              let res = try await session.respond(to: Prompt(content))
              print("Response: \(res.content)")
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
      })
  }
}
