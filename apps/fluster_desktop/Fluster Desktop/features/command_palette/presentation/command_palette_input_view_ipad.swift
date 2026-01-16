//
//  command_palette_input_view_ipad.swift
//  Fluster Desktop
//
//  Created by Andrew on 1/15/26.
//

// NOTE: THis should work, but won't be applicable in the macOS only environment since UIKit is not available.
//import SwiftUI
//import UIKit
//
//struct BackspaceTextField: UIViewRepresentable {
//    var onBackspace: () -> Void
//
//    func makeUIView(context: Context) -> UITextField {
//        let textField = BackspaceDetectingTextField()
//        textField.onBackspace = onBackspace
//        return textField
//    }
//
//    func updateUIView(_ uiView: UITextField, context: Context) {}
//
//    class BackspaceDetectingTextField: UITextField {
//        var onBackspace: (() -> Void)?
//
//        override func deleteBackward() {
//            // This triggers even if text.isEmpty
//            onBackspace?()
//            super.deleteBackward()
//        }
//    }
//}
