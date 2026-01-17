//
//  notes_directory_setting_section.swift
//  Fluster Desktop
//
//  Created by Andrew on 1/16/26.
//

import SwiftUI
import UniformTypeIdentifiers

struct NotesDirSettingSection: View {
  @AppStorage(DesktopAppStorageKeys.notesDirectory.rawValue) private var notesDirectory: String = ""
  @AppStorage(DesktopAppStorageKeys.respectGitIgnore.rawValue) private var respectGitIgnore: Bool =
    true
  @State private var showNotesDirPicker: Bool = false

  var body: some View {
    SettingsSection(title: "File System") {
      VStack(alignment: .leading) {
        Text("Notes Directory")
          .font(.subheadline)
          .foregroundStyle(.secondary)
        HStack {
          TextField(
            text: $notesDirectory, prompt: Text("Notes Directory"),
            label: {
              Text("Notes Directory")
            }
          )
          .textFieldStyle(.squareBorder)
          Button(
            action: {
              showNotesDirPicker = true
            },
            label: {
              Image(systemName: "plus.rectangle.on.folder")
            }
          )
          .buttonStyle(.borderedProminent)
        }
        Text(
          "This is the directory that will be synchronized with your database. This directory can contain any number of folders, each with as many supported files as you like. Unsupported files will simply be ignored. Explicitly set the fsPath in your notes front matter to override it's default location which will be set to the path relative root of this directory."
        )
        .font(.caption)
        .foregroundStyle(.secondary)
        Spacer(minLength: 32)
        HStack(alignment: .center, spacing: 16) {
          Toggle(
            isOn: $respectGitIgnore,
            label: {
            }
          )
          .labelsHidden()
          .toggleStyle(.switch)
          VStack(alignment: .leading) {
            Text("Respect .gitignore")
              .font(.title3)
            Text(
              "If true, files ignored by a .gitignore file within your notes will be ignored by Fluster as well. Be aware that some notes may be visible under file glob based search even when ignored in this manner, but they will not be saved to your database."
            )
            .foregroundStyle(.secondary)
          }
        }
      }
      .fileImporter(
        isPresented: $showNotesDirPicker, allowedContentTypes: [.directory],
        allowsMultipleSelection: false,
        onCompletion: { res in
          switch res {
            case .success(let urls):
              if !urls.isEmpty {
                notesDirectory = urls.first!.absoluteString.replacingOccurrences(
                  of: "file://", with: "")
              }
            case .failure(let error):
              print("File Import Error: \(error.localizedDescription)")
          }
        })
    }
  }
}

#Preview {
  UISettingSection()
}
