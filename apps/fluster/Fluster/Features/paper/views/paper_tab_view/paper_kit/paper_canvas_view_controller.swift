//
//  paper_canvas_view_controller.swift
//  Fluster
//
//  Created by Andrew on 2/22/26.
//

import PaperKit
import SwiftUI
import UIKit

class PaperCanvasViewController: PaperMarkupViewController.Delegate {
  init() {
  }
  func paperMarkupViewControllerDidChangeMarkup(
    _ paperMarkupViewController: PaperMarkupViewController
  ) {
    print("A")
  }

  func paperMarkupViewControllerDidChangeSelection(
    _ paperMarkupViewController: PaperMarkupViewController
  ) {
    print("B")
  }

  func paperMarkupViewControllerDidBeginDrawing(
    _ paperMarkupViewController: PaperMarkupViewController
  ) {
    print("C")
  }

  func paperMarkupViewControllerDidChangeContentVisibleFrame(
    _ paperMarkupViewController: PaperMarkupViewController
  ) {
    print("D")
  }
}
