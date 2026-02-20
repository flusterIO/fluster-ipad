//
//  paper_toolkit_controller.swift
//  Fluster
//
//  Created by Andrew on 2/19/26.
//

import AppKit
import PaperKit
import PencilKit
import SwiftUI

class MacPaperNsViewController: NSViewController, PaperMarkupViewController.Delegate {
  @Binding public var markup: PaperMarkup
  var paperViewController: PaperMarkupViewController!
  var toolbarViewController: MarkupToolbarViewController!

  init(markup: Binding<PaperMarkup>) {
    self._markup = markup
    super.init(nibName: nil, bundle: nil)
  }

  required init?(coder: NSCoder) {
    fatalError("init(coder:) has not been implemented")
  }

  override func loadView() {
    // Create a base view for our container
    self.view = NSView()
  }

  override func viewDidLoad() {
    super.viewDidLoad()

    var features = FeatureSet.latest
    // HDR according to Apple. I'd be lying if I said understood this...
    features.colorMaximumLinearExposure = 4

    // 1. Setup the main Paper canvas
    paperViewController = PaperMarkupViewController(markup: markup, supportedFeatureSet: features)
    addChild(paperViewController)
    view.addSubview(paperViewController.view)

    paperViewController.view.translatesAutoresizingMaskIntoConstraints = false
    NSLayoutConstraint.activate([
      paperViewController.view.leadingAnchor.constraint(equalTo: view.leadingAnchor),
      paperViewController.view.trailingAnchor.constraint(equalTo: view.trailingAnchor),
      paperViewController.view.topAnchor.constraint(equalTo: view.topAnchor),
      paperViewController.view.bottomAnchor.constraint(equalTo: view.bottomAnchor)
    ])

    paperViewController.delegate = self

    // 2. Setup the macOS-specific toolbar for tools and markup insertion
    toolbarViewController = MarkupToolbarViewController(supportedFeatureSet: features)

    // Link the toolbar to the canvas so it knows what to edit
    toolbarViewController.delegate = paperViewController

    addChild(toolbarViewController)
    view.addSubview(toolbarViewController.view)

    // 3. Position the toolbar (e.g., floating at the top center of the canvas)
    toolbarViewController.view.translatesAutoresizingMaskIntoConstraints = false
    NSLayoutConstraint.activate([
      toolbarViewController.view.topAnchor.constraint(equalTo: view.topAnchor, constant: 16),
      toolbarViewController.view.centerXAnchor.constraint(equalTo: view.centerXAnchor)
    ])
  }

  //  private func initializeMarkupModel(with canvasSize: CGSize) {
  //    let bounds = CGRect(origin: .zero, size: canvasSize)
  //    markupModel = PaperMarkup(bounds: bounds)
  //  }
  //  func clearCanvas() {
  //    // Re-initialize the model with a new empty drawing.
  //    initializeMarkupModel(with: canvasSize)
  //    markupModel?.drawing = PKDrawing()
  //
  //    // Update the view controller with the new, empty model.
  //    paperViewController.markup = markupModel
  //  }

  func undoLastAction() {
    paperViewController.undoManager?.undo()
  }

  func paperMarkupViewControllerDidChangeMarkup(
    _ paperMarkupViewController: PaperMarkupViewController
  ) {
    if let markup = paperMarkupViewController.markup {
      self.markup = markup
    }
  }
}
