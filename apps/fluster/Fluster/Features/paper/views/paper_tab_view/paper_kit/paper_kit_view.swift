//
//  paper_kit_view.swift
//  Fluster
//
//  Created by Andrew on 2/22/26.
//

import FlusterData
import PaperKit
import SwiftUI

struct PaperKitView: View {
  @Binding public var editingNote: NoteModel
  @State private var documentData = Data()

  var body: some View {
    GeometryReader { geometry in
      NavigationStack {
        PaperCanvasView(
          canvasData: $documentData,
//          canvasBounds: CGRect(origin: .zero, size: geometry.size)
        )
        .ignoresSafeArea(edges: .bottom)
        .navigationTitle("iPad Markup")
        .navigationBarTitleDisplayMode(.inline)
        .onDisappear {
          // In a real app, you would pass a save action via an Environment
          // key or trigger the Coordinator's save method before dismissing.
          print("Canvas dismissed. Trigger save sequence here.")
        }
      }
    }
  }
}
