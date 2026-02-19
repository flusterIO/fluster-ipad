//
//  paper_view.swift
//  Fluster
//
//  Created by Andrew on 2/17/26.
//

import FlusterData
import PaperKit
import PencilKit
import SwiftUI

struct PaperView: View {
  public var editingNote: NoteModel
  @State private var focusedPageIndex: Int = 0
  var body: some View {
    GeometryReader { geometry in
      Group {
        if focusedPageIndex < editingNote.paper.count {
          let item = editingNote.paper[focusedPageIndex]
          if let markup = try? PaperMarkup(dataRepresentation: item.markup) {
            PaperMarkupView(markup: markup)
          }
        } else {
          Color.clear
        }
      }
      .onChange(
        of: focusedPageIndex,
        {
          Task {
            await handleFocusIndexPageCreation(geometry)
          }
        }
      )
      .task {
        await handleFocusIndexPageCreation(geometry)
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

struct PaperMarkupView: NSViewControllerRepresentable {
  typealias NSViewControllerType = MacPaperContainerViewController
  let markup: PaperMarkup

  func makeNSViewController(context: Context) -> MacPaperContainerViewController {
    return MacPaperContainerViewController(markup: markup)
  }

  func updateNSViewController(_ nsViewController: MacPaperContainerViewController, context: Context)
  {
    print("Updating paper...")
  }

  func makeCoordinator() -> Coordinator {
    Coordinator(self)
  }

  class Coordinator: NSObject {
    var parent: PaperMarkupView
    init(_ parent: PaperMarkupView) { self.parent = parent }

    // Your delegate methods to sync the drawing back to NoteModel
  }
}
