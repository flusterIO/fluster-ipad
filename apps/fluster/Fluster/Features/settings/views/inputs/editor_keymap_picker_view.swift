//
//  editor_keymap_picker_view.swift
//  Fluster
//
//  Created by Andrew on 11/21/25.
//

import FlusterSwift
import SwiftUI
import FlusterData

struct EditorKeymapPickerView: View {
  @AppStorage(AppStorageKeys.editorKeymap.rawValue) private var editorKeymap: EditorKeymap = .base
  @Environment(\.colorScheme) var colorScheme
  @Environment(ThemeManager.self) private var themeManager: ThemeManager

  var body: some View {
    Picker(selection: $editorKeymap, label: Text("Font Size")) {
      ForEach(0..<EditorKeymap.allCases.count) {
        Text(EditorKeymap.allCases[$0].rawValue).tag(
          EditorKeymap.allCases[$0]
        )
      }
    }
    .pickerStyle(.segmented)
    .navigationTitle("Keymap")
  }
}
