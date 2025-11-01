//
//  canvas_view.swift
//  Fluster
//
//  Created by Andrew on 11/1/25.
//

import SwiftUI
import PencilKit


struct CanvasView: UIViewRepresentable {
    let toolPicker = PKToolPicker()
    func makeUIView(context: Context) -> some PKCanvasView {
        let canvasView = PKCanvasView()
        canvasView.drawingPolicy = .anyInput
        toolPicker.setVisible(true, forFirstResponder: canvasView)
        toolPicker.addObserver(canvasView)
        canvasView.becomeFirstResponder()
        return canvasView
    }
    func updateUIView(_ uiView: some PKCanvasView, context: Context) {
        
    }
}
