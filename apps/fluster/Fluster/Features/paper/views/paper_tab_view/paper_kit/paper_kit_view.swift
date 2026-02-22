//
//  paper_kit_view.swift
//  Fluster
//
//  Created by Andrew on 2/22/26.
//

import FlusterData
import FlusterSwift
import PaperKit
import SwiftUI

struct PaperKitView: View {
  @Binding public var editingNote: NoteModel
  @Binding public var focusedPageIndex: Int
  @State private var data: Data = Data()

  var body: some View {
    GeometryReader { geometry in
      if focusedPageIndex <= editingNote.paper.count - 1 {
        if let ep = Binding(Binding($editingNote.paper[focusedPageIndex].markup)) {
          NavigationStack {
            PaperCanvasView(
              canvasData: ep,
              canvasBounds: CGRect(origin: .zero, size: CGSize(width: geometry.size.width, height: UIScreen.current?.bounds.height ?? geometry.size.height))
            )
            .ignoresSafeArea(edges: .bottom)
            .navigationTitle("Page \(focusedPageIndex + 1) of \(editingNote.paper.count)")
            //        .navigationBarTitleDisplayMode(.inline)
            .onDisappear {
              // In a real app, you would pass a save action via an Environment
              // key or trigger the Coordinator's save method before dismissing.
              print("Canvas dismissed. Trigger save sequence here.")
            }
          }
        }
      } else {
        Color.clear.task {
          await handleFocusIndexPageCreation(geometry)
        }
      }
    }
  }

  func handleFocusIndexPageCreation(_ geometry: GeometryProxy) async {
    if focusedPageIndex >= editingNote.paper.count {
      let markup = PaperMarkup(bounds: geometry.frame(in: .local))
      do {
        let data = try await markup.dataRepresentation()
        self.editingNote.paper.append(PaperModel(markup: data))
      } catch {
        print("Error handling paper creation: \(error.localizedDescription)")
      }
    }
  }
}
