//
//  switch_group_view.swift
//  Fluster
//
//  Created by Andrew on 2/18/26.
//

import SwiftUI

struct SwitchGroup: View {
  @Binding public var isOn: Bool
  let title: String
  let desc: String

  // 3. Use the content property in the body of your custom view
  var body: some View {
    HStack(alignment: .center, spacing: 16) {
      Toggle(
        isOn: $isOn,
        label: {
        }
      )
      .labelsHidden()
      .toggleStyle(.switch)
      VStack(alignment: .leading) {
        Text(title)
          .font(.title3)
        Text(
          desc
        )
        .foregroundStyle(.secondary)
      }
    }
  }
}
