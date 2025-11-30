//
//  split_view_view.swift
//  Fluster
//
//  Created by Andrew on 11/30/25.
//

import SwiftUI

public struct SplitView<Vl: View, Vr: View>: View {
    @AppStorage(AppStorageKeys.splitViewEditorSplit.rawValue)
    var splitViewRatio: Double = 0.5
    @Environment(\.colorScheme) private var colorScheme: ColorScheme
    public let left: Vl
    public let right: Vr
    public let height: CGFloat?
    public var onDragStart: () -> Void = {}
    public var onDragEnd: () -> Void = {}
    var screenHeight: CGFloat {
        if let h = UIScreen.current?.bounds.height {
            return h
        }
        return 0
    }

    public init(
        @ViewBuilder left: () -> Vl,
        @ViewBuilder right: () -> Vr,
        height: CGFloat? = nil,
        onDragStart: (() -> Void)? = nil,
        onDragEnd: (() -> Void)? = nil
    ) {
        self.left = left()
        self.right = right()
        self.height = height
        if onDragEnd != nil {
            self.onDragEnd = onDragEnd!
        }
        if onDragStart != nil {
            self.onDragStart = onDragStart!
        }
    }
    public var body: some View {
        GeometryReader { rect in
            ZStack {
                colorScheme == .dark ? Color.black : Color.white
                HStack {
                    left
                        .frame(
                            width: rect.size.width * splitViewRatio,
                            height: screenHeight - rect.safeAreaInsets.top
                        )
                    SplitViewDividerView(
                        splitViewRatio: $splitViewRatio,
                        rect: rect,
                        onDragStart: onDragStart,
                        onDragEnd: onDragEnd
                    )
                    right
                        .frame(
                            width: rect.size.width * (1 - splitViewRatio),
                            height: screenHeight - rect.safeAreaInsets.top
                        )
                        .ignoresSafeArea(edges: .bottom)
                }
            }
        }
        .coordinateSpace(name: "splitView")
    }
}

#Preview {
    SplitView(
        left: {
            Text("Left")
        },
        right: {
            Text("Right")
        }
    )
}
