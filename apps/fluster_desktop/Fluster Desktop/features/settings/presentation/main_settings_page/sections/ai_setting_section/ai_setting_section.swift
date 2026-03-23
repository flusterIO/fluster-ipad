//
//  ai_setting_section.swift
//  Fluster
//
//  Created by Andrew on 3/23/26.
//

import FlusterData
import Foundation
import FlusterSwift
import SwiftUI

struct AISettingSectionView: View {
  @AppStorage(AppStorageKeys.embeddedCslFile.rawValue) private var embeddedCslFile:
    EmbeddedCslFileSwift = .apa
  var body: some View {
    SettingsSection(
      title: "Artificial Intelligence",
      content: {
          VStack(alignment: .leading) {
              Text("Preferred Name")
                  .font(.headline)
              UserNameSettingInput()
              Text("This can be any name you like as this is only used for personalization through AI, but it should make sense in the following sentence: 'Hello, this is my friend ...'.")
                  .font(.caption)
                  .foregroundStyle(.secondary)
          }
          .frame(maxWidth: .infinity, alignment: .leading)
      })
  }
}
