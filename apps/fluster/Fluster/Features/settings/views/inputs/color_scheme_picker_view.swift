//
//  color_scheme_picker_view.swift
//  Fluster
//
//  Created by Andrew on 11/17/25.
//

import FlusterData
import FlusterSwift
import SwiftUI

struct ColorSchemePickerView: View {
  @Binding var scheme: ColorSchemeSelection
  @Environment(\.colorScheme) var colorScheme
  @Environment(ThemeManager.self) private var themeManager: ThemeManager

  var body: some View {
    Picker(selection: $scheme, label: Text("Editor Theme")) {
      ForEach(ColorSchemeSelection.allCases, id: \.rawValue) { item in
        Text(item.rawValue).tag(
          item
        )
      }
    }
    .pickerStyle(.segmented)
  }
}

#Preview {
  EditorThemePickerView(theme: .constant(.githubDark), title: "Test title")
}
