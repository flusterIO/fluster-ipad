//
//  command_palette_input_view_mac.swift
//  Fluster Desktop
//
//  Created by Andrew on 1/15/26.
//

import AppKit
import SwiftUI

struct CommandPaletteInputViewMac: NSViewRepresentable {
  @AppStorage(DesktopAppStorageKeys.colorScheme.rawValue) private var selectedTheme: AppTheme = .dark

  @Binding var text: String
  var onBackspace: () -> Void
  var onTab: () -> Void
  var onBackTab: () -> Void
  var onEnter: () -> Void
  var onDownArrow: () -> Void
  var onUpArrow: () -> Void

  func makeNSView(context: Context) -> NSTextField {
    let textField = NSTextField()
    textField.delegate = context.coordinator
    textField.isEditable = true
    textField.stringValue = text
    textField.backgroundColor = .clear
    textField.focusRingType = .none
    textField.isBordered = false
    return textField
  }

  func updateNSView(_ nsView: NSTextField, context: Context) {
    nsView.stringValue = text
  }

  func makeCoordinator() -> Coordinator { Coordinator(onBackspace: onBackspace, parent: self) }

  class Coordinator: NSObject, NSTextFieldDelegate {
    var onBackspace: () -> Void
    var parent: CommandPaletteInputViewMac
    init(onBackspace: @escaping () -> Void, parent: CommandPaletteInputViewMac) {
      self.onBackspace = onBackspace
      self.parent = parent
    }

    func controlTextDidChange(_ obj: Notification) {
      if let textField = obj.object as? NSTextField {
        // Update the SwiftUI binding (which updates the parent's @State)
        parent.text = textField.stringValue
      }
    }
    func control(_ control: NSControl, textView: NSTextView, doCommandBy commandSelector: Selector)
      -> Bool
    {
      switch commandSelector {
        case #selector(NSResponder.deleteBackward(_:)):
          parent.onBackspace()
        case #selector(NSResponder.insertTab(_:)):
          parent.onTab()
          return true
        case #selector(NSResponder.insertBacktab(_:)):
          parent.onBackTab()
          return true
        case #selector(NSResponder.insertNewline(_:)):
          parent.onEnter()
          return true
        case #selector(NSResponder.moveUp(_:)):
          parent.onUpArrow()
          return true
        case #selector(NSResponder.moveDown(_:)):
          parent.onDownArrow()
          return true
        default:
          return false
      }
      return false
    }
  }
}
