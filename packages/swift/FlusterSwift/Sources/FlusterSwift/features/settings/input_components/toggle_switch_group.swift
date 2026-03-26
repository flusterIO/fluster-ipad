//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 3/26/26.
//

import SwiftUI

public struct ToggleSwitchGroup: View {
    @Binding public var value: Bool
    public let title: String
    public let subtitle: String
    public let label: Label<Text, Image>
    public init(value: Binding<Bool>, title: String, subtitle: String, label: Label<Text, Image>) {
        self._value = value
        self.title = title
        self.subtitle = subtitle
        self.label = label
    }
    public var body: some View {
        HStack(alignment: .center, spacing: 16) {
          Toggle(
            isOn: $value,
            label: {
                label
            }
          )
          .labelsHidden()
          .toggleStyle(.switch)
          VStack(alignment: .leading) {
            Text(title)
              .font(.title3)
            Text(
                subtitle
            )
            .foregroundStyle(.secondary)
          }
        }
    }
}
