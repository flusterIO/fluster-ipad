//
//  paper_view.swift
//  Fluster
//
//  Created by Andrew on 2/17/26.
//

import PaperKit
import SwiftUI

struct PaperView: View {
  @Binding private var paperMarkup: PaperMarkup?
  var body: some View {
    GeometryReader { geometry in
      NavigationStack {
        Group {
          if let markup = paperMarkup {
            PaperMarkupView(markup: markup)
          } else {
            Color.clear
          }
        }
      }
      .onAppear {
        paperMarkup = PaperMarkup(bounds: geometry.frame(in: .local))
      }
    }
  }
}

struct PaperMarkupView: NSViewControllerRepresentable {
  typealias NSViewControllerType = PaperMarkupViewController
  let markup: PaperMarkup
  func makeNSViewController(context: Context) -> PaperMarkupViewController {
    return PaperMarkupViewController(markup: markup, supportedFeatureSet: .latest)
  }

  func updateNSViewController(_ nsViewController: PaperMarkupViewController, context: Context) {
    print("Updating paper...")
  }

  func makeCoordinator() -> Coordinator {
    Coordinator(self)
  }

  class Coordinator: NSObject {
    var parent: PaperMarkupView
    init(_ parent: PaperMarkupView) { self.parent = parent }

    func paperMarkupViewControllerDidUpdateDrawing(_ controller: PaperMarkupViewController) {
      // Sync back to the source of truth
//      DispatchQueue.main.async {
//        self.parent.drawing = controller.markup.drawing
//      }
    }
  }
}
