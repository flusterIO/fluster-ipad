//
//  paper_view.swift
//  Fluster
//
//  Created by Andrew on 11/1/25.
//

import FlusterSwift
import PencilKit
import SwiftUI

struct PaperView: View {
  @Binding var toolbar: PKToolPicker
  @Binding var drawingData: Data
  @State private var canvasView = PKCanvasView()
  @Binding var activeTab: IpadMainViewTab

  var body: some View {
    CanvasView(
      toolPicker: $toolbar,
      drawingData: $drawingData,
      canvasView: $canvasView
    )
    .onChange(
      of: activeTab,
      {
        if activeTab == .paper {
          toolbar.setVisible(true, forFirstResponder: canvasView)
          toolbar.addObserver(canvasView)
          canvasView.becomeFirstResponder()
        }
      }
    )
  }
}
