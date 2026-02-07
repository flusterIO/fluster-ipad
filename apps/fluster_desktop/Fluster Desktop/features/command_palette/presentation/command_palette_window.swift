//
//  command_palette_window.swift
//  Fluster
//
//  Created by Andrew on 2/7/26.
//

import AppKit
import Foundation
import SwiftUI

let COMMAND_PALETTE_WIDTH: Int = 600
let COMMAND_PALETTE_HEIGHT: Int = 400

class CommandPalettePanel: NSPanel {
  init<Content: View>(rootView: Content) {
    super.init(
      contentRect: NSRect(x: 0, y: 0, width: COMMAND_PALETTE_WIDTH, height: COMMAND_PALETTE_HEIGHT),
      styleMask: [.nonactivatingPanel, .fullSizeContentView, .resizable, .borderless, .utilityWindow],
      backing: .buffered,
      defer: false
    )
    self.titleVisibility = .hidden
    self.hasShadow = true
    self.level = .mainMenu
    self.isFloatingPanel = true
    self.collectionBehavior = [.canJoinAllSpaces, .fullScreenAuxiliary]
    self.titlebarSeparatorStyle = .none
    self.titlebarAppearsTransparent = true
    self.standardWindowButton(.closeButton)?.isHidden = true
    self.standardWindowButton(.miniaturizeButton)?.isHidden = true
    self.standardWindowButton(.zoomButton)?.isHidden = true
    //
    //    // Hide the close/minimize buttons for a clean look
    //
    //    self.contentView = hostingView

    self.isOpaque = false
    self.backgroundColor = .clear

    // 1. Create the hosting view
    let hostingView = NSHostingView(rootView: rootView.ignoresSafeArea())
    hostingView.translatesAutoresizingMaskIntoConstraints = false
    hostingView.layer?.backgroundColor = .clear
    // 2. Add to content view
    guard let contentView = self.contentView else { return }
    contentView.addSubview(hostingView)

    // 3. Pin edges to the window perfectly
    NSLayoutConstraint.activate([
      hostingView.topAnchor.constraint(equalTo: contentView.topAnchor),
      hostingView.bottomAnchor.constraint(equalTo: contentView.bottomAnchor),
      hostingView.leadingAnchor.constraint(equalTo: contentView.leadingAnchor),
      hostingView.trailingAnchor.constraint(equalTo: contentView.trailingAnchor)
    ])
  }

  // Ensure the panel can become the key window to receive text input
  override var canBecomeKey: Bool { true }
}
