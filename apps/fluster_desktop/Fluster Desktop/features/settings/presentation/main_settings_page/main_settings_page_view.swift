//
//  main_settings_page.swift
//  fluster_desktop
//
//  Created by Andrew on 1/14/26.
//

import SwiftUI

struct MainSettingsPageView: View {
  @State private var theme: Theme = .system
  @State private var font: FontChoice = .serif
  @State private var renderMath = true
  @State private var cloudSync = true
  @State private var localBackup = false
  @State private var equationEditor = true
  @State private var citationManager = false
  @State private var codeHighlighting = true
  @State private var textSize: Double = 16
  @State private var highContrast = false

  var body: some View {
    HStack(alignment: .center) {
      ScrollView {
        VStack(alignment: .leading, spacing: 24) {
          // Header
          HStack(spacing: 12) {
            Image("flusterIcon")
              .resizable()
              .foregroundStyle(.tint)
              .aspectRatio(contentMode: .fit)
              .frame(width: 48, height: 48, alignment: .center)
            VStack(alignment: .leading) {
              Text("Fluster")
                .font(.title)
                .fontWeight(.bold)
              Text("Settings")
                .font(.headline)
                .foregroundStyle(.secondary)
            }
            Spacer()
          }
          .padding(.horizontal)
          .frame(maxWidth: .infinity)

          // Account & Profile
          SettingsSection(title: "Account & Profile") {
            HStack {
              Circle()
                .fill(.gray.opacity(0.2))
                .frame(width: 52, height: 52)
                .overlay(Text("A").font(.title2))
              VStack(alignment: .leading) {
                Text("Andrew Smith")
                  .font(.headline)
                Text("andrew@email.com")
                  .font(.subheadline)
                  .foregroundStyle(.secondary)
              }
            }
          }

          NotesDirSettingSection()

          AutoSettingSettingSection()

          UISettingSection()

          EditorSettingSectionView()
        }
        .padding(.vertical, 32)
      }
      .scrollIndicators(.hidden)
      .frame(maxWidth: 1080)
    }
    .formStyle(.automatic)
  }
}

// MARK: - Supporting Types and Views
private enum Theme: String, CaseIterable {
  case system, light, dark
  var description: String {
    switch self {
      case .system: return "System"
      case .light: return "Light"
      case .dark: return "Dark"
    }
  }
}

private enum FontChoice: String, CaseIterable {
  case serif, sans, mono
  var description: String {
    switch self {
      case .serif: return "Serif"
      case .sans: return "Sans Serif"
      case .mono: return "Monospace"
    }
  }
}

#Preview {
  MainSettingsPageView()
}
