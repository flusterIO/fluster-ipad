//
//  paper_view_controller.swift
//  Fluster
//
//  Created by Andrew on 2/17/26.
//

import AppKit
import PaperKit


class DocumentViewController: NSViewController {
   @ViewLoading private var paperViewController: PaperMarkupViewController
   @ViewLoading private var markupToolbarViewController: MarkupToolbarViewController
   @ViewLoading private var paperMarkup: PaperMarkup


   override func viewDidLoad() {
       super.viewDidLoad()


       //Initialize the data model.
       paperMarkup = PaperMarkup(bounds: view.bounds)
       
       // Create the markup view controller.
       paperViewController = PaperMarkupViewController(markup: paperMarkup, supportedFeatureSet: .latest)
       
       // Add as a child view controller, and set the layout.
       view.addSubview(paperViewController.view)
       addChild(paperViewController)
       setupLayoutConstraints()
       
       // Set the toolbar's delegate to the markup controller, and add it to the hierarchy.
       setupToolbarViewController()
   }
   
   private func setupLayoutConstraints() {
       paperViewController.view.translatesAutoresizingMaskIntoConstraints = false
       
       NSLayoutConstraint.activate([
           paperViewController.view.topAnchor.constraint(equalTo: view.topAnchor),
           paperViewController.view.bottomAnchor.constraint(equalTo: view.bottomAnchor),
           paperViewController.view.leadingAnchor.constraint(equalTo: view.leadingAnchor),
           paperViewController.view.trailingAnchor.constraint(equalTo: view.trailingAnchor)
       ])
   }
   
   private func setupToolbarViewController() {
       markupToolbarViewController = MarkupToolbarViewController(supportedFeatureSet: .latest)
       markupToolbarViewController.delegate = paperViewController
       view.addSubview(markupToolbarViewController.view)
       addChild(markupToolbarViewController)
   }
}
