//
//  canvas_view.swift
//  Fluster
//
//  Created by Andrew on 11/1/25.
//

import PencilKit
import SwiftUI

struct CanvasView: UIViewRepresentable {
  @Binding var toolPicker: PKToolPicker
  @Binding var drawingData: Data
  @Binding var canvasView: PKCanvasView

  func makeUIView(context: Context) -> PKCanvasView {
    canvasView.drawingPolicy = .anyInput
    canvasView.delegate = context.coordinator
    canvasView.minimumZoomScale = 0.01
    canvasView.maximumZoomScale = 1000.0
    canvasView.zoomScale = 1.0
    canvasView.isScrollEnabled = true
    toolPicker.setVisible(true, forFirstResponder: canvasView)
    toolPicker.addObserver(canvasView)
    canvasView.becomeFirstResponder()
    return canvasView
  }

  func showToolbar() {
    toolPicker.setVisible(true, forFirstResponder: canvasView)
    toolPicker.addObserver(canvasView)
    canvasView.becomeFirstResponder()
  }

  func updateUIView(_ uiView: PKCanvasView, context: Context) {
    // 1. Attempt to deserialize the data into a PKDrawing
    if let drawing = try? PKDrawing(data: drawingData) {
      // 2. Only update the canvas if the drawing has actually changed
      // This prevents infinite loops where updateUIView triggers a save, which triggers updateUIView...
      if uiView.drawing != drawing {
        uiView.drawing = drawing
      }
    }
  }

  func makeCoordinator() -> Coordinator {
    Coordinator(self)
  }

  // The Coordinator listens to the PKCanvasView delegates
  class Coordinator: NSObject, PKCanvasViewDelegate {
    var parent: CanvasView

    init(_ parent: CanvasView) {
      self.parent = parent
    }

    // Triggered whenever the user lifts their finger/pencil
    func canvasViewDrawingDidChange(_ canvasView: PKCanvasView) {
      // Convert the drawing back to Data and update the binding
      parent.drawingData = canvasView.drawing.dataRepresentation()
    }
  }
}
