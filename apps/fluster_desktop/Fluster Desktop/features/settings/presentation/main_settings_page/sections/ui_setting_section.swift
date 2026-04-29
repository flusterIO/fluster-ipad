//
//  ui_setting_section.swift
//  Fluster Desktop
//
//  Created by Andrew on 1/15/26.
//

import FlusterData
import FlusterSwift
import SwiftUI

struct UISettingSection: View {
  @AppStorage(AppStorageKeys.colorScheme.rawValue) private var selectedTheme: AppTheme =
    .dark

  var body: some View {
    SettingsSection(title: "UI Customization") {
      VStack(alignment: .leading, spacing: 64) {
        Picker("Color Scheme", selection: $selectedTheme) {
          ForEach(AppTheme.allCases, id: \.self) { theme in
            Text(theme.rawValue).tag(theme.rawValue)
          }
        }
        .pickerStyle(.segmented)
      }.frame(maxWidth: .infinity, alignment: .leading)
      CodeBlockThemePickerLight()
      CodeBlockThemePickerDark()
    }
  }
}

#Preview {
  UISettingSection()
}
