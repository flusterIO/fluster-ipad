//
//  canvas_view.swift
//  Fluster
//
//  Created by Andrew on 11/1/25.
//

import SwiftUI
import PencilKit


struct CanvasView: UIViewRepresentable {
    @Binding var toolPicker:  PKToolPicker
    @Binding var canvasView: PKCanvasView
    var drawing: PKDrawing

    
    func makeUIView(context: Context) -> some PKCanvasView {
        canvasView.drawingPolicy = .anyInput
        canvasView.drawing = drawing
        toolPicker.setVisible(true, forFirstResponder: canvasView)
        toolPicker.addObserver(canvasView)
        canvasView.becomeFirstResponder()
        return canvasView
    }
    func updateUIView(_ uiView: some PKCanvasView, context: Context) {
        // toolPicker.setVisible(true, forFirstResponder: uiView)
    }
}
