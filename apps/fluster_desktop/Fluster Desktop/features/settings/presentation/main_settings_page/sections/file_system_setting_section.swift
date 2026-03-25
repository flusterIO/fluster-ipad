//
//  notes_directory_setting_section.swift
//  Fluster Desktop
//
//  Created by Andrew on 1/16/26.
//

import FlusterData
import SwiftUI
import UniformTypeIdentifiers

struct NotesDirSettingSection: View {
  @AppStorage(AppStorageKeys.notesDirectory.rawValue) private var notesDirectory: String = ""
  @AppStorage(AppStorageKeys.respectGitIgnore.rawValue) private var respectGitIgnore: Bool =
    true
  @AppStorage(AppStorageKeys.defaultNoteView.rawValue) private var defaultNoteView:
    DefaultNoteView = .markdown
  @State private var showNotesDirPicker: Bool = false

  var body: some View {
    SettingsSection(title: "Notes", subtitle: "The file system re-integration is still a work in progress after Fluster was migrated from a primarily file-system based storage layer to a database driven system. While the database will remain the single source of truth, a thorough and complete integration with the user's file system will be available soon.") {
      VStack(alignment: .leading) {
        Text("Notes Directory")
          .font(.headline)
        VStack(alignment: .leading, spacing: 16) {
          VStack(alignment: .leading) {
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
          }
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
          VStack(alignment: .leading) {
            Text("Default View")
              .font(.headline)
            Picker(
              selection: $defaultNoteView,
              content: {
                ForEach(DefaultNoteView.allCases, id: \.rawValue) { item in
                  Text(item.rawValue).tag(item)
                }
              },
              label: {
                Text("Default View")
              }
            )
            .labelsHidden()
            Text("The view that will be shown when a not is initially selected.").font(.caption)
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
