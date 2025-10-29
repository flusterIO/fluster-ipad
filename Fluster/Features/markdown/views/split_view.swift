//
//  split_view.swift
//  Fluster
//
//  Created by Andrew on 10/29/25.
//

import SwiftUI

struct EditorSplitView: View {
    @State private var leftPaneWidthFraction: CGFloat = 0.5
    @GestureState private var dragOffset: CGFloat = 0

    private let minPaneWidthFraction: CGFloat = 0.1

    var body: some View {
        GeometryReader { geometry in
            let adjustedLeftPaneWidth = (geometry.size.width * leftPaneWidthFraction) + dragOffset
            let lowerBound = geometry.size.width * minPaneWidthFraction
            let upperBound = geometry.size.width * (1 - minPaneWidthFraction)
            
            let clampedLeftPaneWidth = max(lowerBound, min(upperBound, adjustedLeftPaneWidth))

            HStack(spacing: 0) {
                // Left View
                MarkdownEditorView()
                    .frame(width: clampedLeftPaneWidth)

                // Draggable Splitter
                Rectangle()
                    .fill(Color.gray.opacity(0.5))
                    .frame(width: 8)
                    .contentShape(Rectangle())
                    .gesture(
                        DragGesture(minimumDistance: 0)
                            .updating($dragOffset) { value, state, _ in
                                state = value.translation.width
                            }
                            .onEnded { value in
                                let newWidth = (geometry.size.width * leftPaneWidthFraction) + value.translation.width
                                let finalWidth = max(lowerBound, min(upperBound, newWidth))
                                self.leftPaneWidthFraction = finalWidth / geometry.size.width
                            }
                    )

                // Right View
                Color.green
            }
        }
    }
}


#Preview {
    EditorSplitView()
}
