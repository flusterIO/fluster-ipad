//
//  ui_setting_section.swift
//  Fluster Desktop
//
//  Created by Andrew on 1/15/26.
//

import SwiftUI

struct UISettingSection: View {
  @AppStorage(DesktopAppStorageKeys.colorScheme.rawValue) private var selectedTheme: AppTheme = .dark

  var body: some View {
    SettingsSection(title: "UI Customization") {
      Picker("Color Scheme", selection: $selectedTheme) {
        ForEach(AppTheme.allCases, id: \.self) { theme in
          Text(theme.rawValue).tag(theme.rawValue)
        }
      }
      .pickerStyle(.segmented)
    }
  }
}

#Preview {
  UISettingSection()
}
