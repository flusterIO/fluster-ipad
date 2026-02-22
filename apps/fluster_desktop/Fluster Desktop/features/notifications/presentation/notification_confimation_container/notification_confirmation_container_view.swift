//
//  notification_confirmation_container_view.swift
//  Fluster
//
//  Created by Andrew on 2/21/26.
//

import SwiftUI

struct NotificationConfirmationContainerView<Content: View>: View {
  @Environment(\.dismiss) private var dismiss
  let content: Content
  let buttonText: String
  init(buttonText: String = "Go back", @ViewBuilder content: () -> Content) {
    self.buttonText = buttonText
    self.content = content()
  }
  var body: some View {
    VStack(alignment: .trailing) {
      content
        .padding(.horizontal)
        .padding(.top)
      Button(
        action: {
          dismiss()
        },
        label: {
          Text(buttonText)
        }
      )
      .frame(alignment: .trailing)
      .padding()
    }
  }
}
