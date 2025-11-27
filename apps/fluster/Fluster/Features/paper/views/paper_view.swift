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
    @Binding var drawingData: Data
    @State private var canvasView = PKCanvasView()
    @Binding var activeTab: MainViewTab

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
                    print("Showing toolbar")
                    toolbar.setVisible(true, forFirstResponder: canvasView)
                    toolbar.addObserver(canvasView)
                    canvasView.becomeFirstResponder()
                }
            }
        )
    }
}
