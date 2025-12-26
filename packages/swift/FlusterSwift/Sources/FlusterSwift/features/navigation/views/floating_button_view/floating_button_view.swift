//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/9/25.
//

import SwiftUI

public struct FloatingButtonItem: Identifiable {
  public let id: String
  public let systemImage: String
  public let action: () -> Void
  public init(id: String, systemImage: String, action: @escaping () -> Void) {
    self.id = id
    self.systemImage = systemImage
    self.action = action
  }
}

public struct FloatingButtonView: View {
  @Namespace private var floatingButtonNamespace
  @State private var open: Bool = false
  public let buttons: [FloatingButtonItem]
  public init(buttons: [FloatingButtonItem]) {
    self.buttons = buttons
  }
  public var body: some View {
    GlassEffectContainer(spacing: 16) {
      VStack(spacing: 16) {
        if open {
          ForEach(buttons) { btn in
            Image(systemName: btn.systemImage)
              .font(.system(size: 36))
              .padding()
              .glassEffect(in: .rect(cornerRadius: 16))
              .onTapGesture {
                btn.action()
                withAnimation {
                  open.toggle()
                }
              }
          }
        }
        Image(systemName: "xmark")
          .rotationEffect(
            open ? Angle(degrees: 0) : Angle(degrees: 45)
          )
          .onTapGesture {
            withAnimation {
              open.toggle()
            }
          }
          .font(.system(size: 36))
          .padding()
          .glassEffect(in: .rect(cornerRadius: 16))
          .glassEffectID(
            "floatingButtonToggle",
            in: floatingButtonNamespace
          )
      }
    }
  }
}

#Preview {
  FloatingButtonView(buttons: [
    FloatingButtonItem(
      id: "myButton",
      systemImage: "trash",
      action: {
        print("Here")
      }
    ),
    FloatingButtonItem(
      id: "myOtherButton",
      systemImage: "house",
      action: {
        print("Here too")
      }
    )
  ])
  .preferredColorScheme(.light)
}
