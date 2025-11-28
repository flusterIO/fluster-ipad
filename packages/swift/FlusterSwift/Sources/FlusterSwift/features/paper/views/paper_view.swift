//
//  paper_view.swift
//  Fluster
//
//  Created by Andrew on 11/1/25.
//

import PencilKit
import SwiftUI

public struct PaperView: View {
    @State private var canvasView = PKCanvasView()
    @Binding var toolbar: PKToolPicker
    @Binding var drawingData: Data
    @Binding var activeTab: IpadMainViewTab
    
    public init(toolbar: Binding<PKToolPicker>, drawingData: Binding<Data>, activeTab: Binding<IpadMainViewTab>) {
        self._toolbar = toolbar
        self._drawingData = drawingData
        self._activeTab = activeTab
    }

    public var body: some View {
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
