//
//  split_view_divider_view.swift
//  Fluster
//
//  Created by Andrew on 11/30/25.
//

import SwiftUI

struct SplitViewDividerView: View {
  @Environment(ThemeManager.self) private var themeManager: ThemeManager
  @State private var dragging: Bool = false
  @Binding var splitViewRatio: Double
  let rect: GeometryProxy
  let handleWidth: CGFloat = 8
  let onDragStart: (() -> Void)?
  let onDragEnd: (() -> Void)?

  var body: some View {
    ZStack {
      // Invisible wide touch area for easier grabbing
      Rectangle()
        .fill(Color.clear)
        .frame(width: handleWidth)

      // Visible thin line
      Capsule()
        .fill(themeManager.theme.muted_foreground)
        .frame(width: 4, height: 40)
    }
    .gesture(
      DragGesture(
        coordinateSpace: .named("splitView")
      )
      .onChanged { drag in
        if !dragging && onDragStart != nil {
          onDragStart!()
        }
        if !dragging {
          dragging = true
          if let _onDragStart = onDragStart {
            _onDragStart()
          }
        }
        splitViewRatio = getSplitViewDimensions(
          drag: drag,
          rect: rect
        )
      }
      .onEnded { ended in
        if let _onDragEnd = onDragEnd {
          _onDragEnd()
        }
        dragging = false
      }
    )
  }
}
