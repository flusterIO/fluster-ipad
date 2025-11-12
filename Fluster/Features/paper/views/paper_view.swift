//
//  paper_view.swift
//  Fluster
//
//  Created by Andrew on 11/1/25.
//

import PencilKit
import SwiftUI

struct PaperView: View {
    @Binding var toolbar: PKToolPicker
    @Binding var canvasView: PKCanvasView
    var drawing: PKDrawing

    var body: some View {
        CanvasView(
            toolPicker: $toolbar,
            canvasView: $canvasView,
            drawing: drawing
        )
    }
}
