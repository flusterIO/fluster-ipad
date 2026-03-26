//
//  search_setting_section_view.swift
//  Fluster
//
//  Created by Andrew on 3/26/26.
//

import FlusterData
import FlusterSwift
import SwiftUI

struct SearchSettingSectionView: View {
  @AppStorage(AppStorageKeys.storePlainText.rawValue) private var storePlainText: Bool = true
  var body: some View {
    SettingsSection(
      title: "Search", subtitle: nil,
      content: {
        ToggleSwitchGroup(
          value: $storePlainText, title: "Store Plain Text",
          subtitle:
            "This will improve the functionality of full-text search at the cost of more storage space as italic, bold and other markdown wrapped text won't match their plain-text counterpart unless a copy is stored alongside the valid note.",
          label: Label(
            title: {
              Text("Store Plain Text")
            },
            icon: {
              Image(systemName: "swiftdata")
            })
        )
      }
    )
  }
}

#Preview {
  EditorSettingSectionView()
}
