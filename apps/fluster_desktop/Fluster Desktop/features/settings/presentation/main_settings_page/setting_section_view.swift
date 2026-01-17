//
//  setting_section_view.swift
//  Fluster Desktop
//
//  Created by Andrew on 1/15/26.
//

import SwiftUI

public struct SettingsSection<Content: View>: View {
  let title: String
  let subtitle: String?
  let content: Content
  init(title: String, subtitle: String? = nil, @ViewBuilder content: () -> Content) {
    self.title = title
    self.subtitle = subtitle
    self.content = content()
  }
  public var body: some View {
    VStack(alignment: .leading) {
      Text(title)
        .font(.title2)
      if subtitle != nil {
        Text(subtitle!)
          .font(.caption)
          .foregroundStyle(.secondary)
      }
      Spacer(minLength: 12)
      VStack(alignment: .leading, spacing: 10) {
        content
      }
      .frame(maxWidth: .infinity)
      .padding()
      .clipShape(RoundedRectangle(cornerRadius: 0, style: .continuous))
      .glassEffect(in: .rect(cornerRadius: 16))
    }
    .padding(.horizontal)
  }
}
