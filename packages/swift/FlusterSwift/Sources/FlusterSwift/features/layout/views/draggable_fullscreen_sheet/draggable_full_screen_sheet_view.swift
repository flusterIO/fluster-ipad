//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/20/25.
//

import SwiftUI


#if os(iOS)
public struct FullScreenSheetDraggableView<Content: View>: View {
  @State private var fullScreenCoverDragDrag: CGFloat = 0
  @State private var fullScreenCoverOpacity: CGFloat = 1
  @Binding var open: Bool
  @Binding var fullScreenCover: MainFullScreenCover?
  let content: Content
  public init(
    open: Binding<Bool>, @ViewBuilder content: () -> Content,
    fullScreenCover: Binding<MainFullScreenCover?>
  ) {
    self._open = open
    self._fullScreenCover = fullScreenCover
    self.content = content()
  }
  public var body: some View {
    ZStack {
      NavigationStack {
        content
          .toolbar {
            ToolbarItem(
              placement: .topBarLeading,
              content: {
                Image(systemName: "xmark")
                  .onTapGesture {
                    fullScreenCover = nil
                  }
              })
          }
      }
    }
    .interactiveDismissDisabled(true)
    .opacity(fullScreenCoverOpacity)
    .offset(y: fullScreenCoverDragDrag)
    .gesture(
      DragGesture()
        .onChanged { drag in
          withAnimation {
            fullScreenCoverDragDrag =
              drag.translation.height
            if drag.translation.height < 100 {
              fullScreenCoverOpacity =
                (100 - fullScreenCoverDragDrag)
                / 100
            } else {
              fullScreenCoverOpacity = 0
            }
          }
        }
        .onEnded { drag in
          withAnimation {
            if drag.translation.height > 100 {
              open = false
              fullScreenCoverOpacity = 0
            } else {
              fullScreenCoverDragDrag = 0
              fullScreenCoverOpacity = 1
            }
          }
        }
    )
  }
}
#endif
