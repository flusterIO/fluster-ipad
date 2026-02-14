//
//  split_view_view.swift
//  Fluster
//
//  Created by Andrew on 11/30/25.
//

import SwiftUI

#if os(iOS)
public enum SplitViewSide {
  case left, right, none
}

public struct SplitView<Vl: View, Vr: View>: View {
  @State private var leftHidden: Bool = false
  @State private var rightHidden: Bool = false
  @Environment(\.colorScheme) private var colorScheme: ColorScheme
  public let left: Vl
  public let right: Vr
  @Binding var splitViewRatio: Double
  public let height: CGFloat?
  public var onDragStart: () -> Void = {}
  public var onDragEnd: () -> Void = {}
  public var hideSide: SplitViewSide?

  var screenHeight: CGFloat {
    if let h = UIScreen.current?.bounds.height {
      return h
    }
    return 0
  }

  public init(
    @ViewBuilder left: () -> Vl,
    @ViewBuilder right: () -> Vr,
    splitViewRatio: Binding<Double>,
    height: CGFloat? = nil,
    onDragStart: (() -> Void)? = nil,
    onDragEnd: (() -> Void)? = nil,
    hideSide: SplitViewSide?,
  ) {
    self.left = left()
    self.right = right()
    self._splitViewRatio = splitViewRatio
    self.height = height
    if onDragEnd != nil {
      self.onDragEnd = onDragEnd!
    }
    if onDragStart != nil {
      self.onDragStart = onDragStart!
    }
    self.hideSide = hideSide
  }
  public var body: some View {
    GeometryReader { rect in
      ZStack {
        colorScheme == .dark ? Color.black : Color.white
        HStack {
          if hideSide != .left {
            left
              .frame(
                width: hideSide == .left
                  ? 0 : rect.size.width * splitViewRatio,
                height: screenHeight - rect.safeAreaInsets.top
              )
          }
          SplitViewDividerView(
            splitViewRatio: $splitViewRatio,
            rect: rect,
            onDragStart: onDragStart,
            onDragEnd: onDragEnd
          )
          right
            .frame(
              width: hideSide == .right
                ? 0 : rect.size.width * (1 - splitViewRatio),
              height: screenHeight - rect.safeAreaInsets.top
            )
            .ignoresSafeArea(edges: .bottom)
        }
      }
    }
    .coordinateSpace(name: "splitView")
  }
  public func hideSide(direction: SplitViewSide) {
    withAnimation(.easeInOut) {
      self.splitViewRatio = direction == .left ? 0 : 1
    }
  }
}

#Preview {
  SplitView(
    left: {
      Text("Left")
    },
    right: {
      Text("Right")
    },
    splitViewRatio: .constant(0.5),
    hideSide: .none
  )
}
#endif
