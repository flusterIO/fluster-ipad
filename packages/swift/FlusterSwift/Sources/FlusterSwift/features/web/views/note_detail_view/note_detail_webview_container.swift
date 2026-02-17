//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/5/25.
//

import Combine
import FlatBuffers
import FlusterData
import SwiftUI
import WebKit

#if os(iOS)
  @MainActor
  public final class NoteDetailWebviewContainer: WebviewContainer<
    NoteDetailWebviewEvents
  >
  {
    public func setNoteDetails(note: NoteModel) {
      let bytes = noteModel.toNoteDetailsByteArray()
      self.runJavascript(
        """
        window.dispatchEvent(
            new CustomEvent("\(NoteDetailWebviewEvents.setNoteDetails.rawValue)", {
                detail: \(bytes),
            }),
        );
        """
      )
    }

    public func setInitialData(
      colorScheme: ColorScheme,
      webviewTheme: WebViewTheme,
      fontSize: WebviewFontSize,
      note: NoteModel
    ) {
      self.applyWebViewColorScheme(
        darkMode: colorScheme == .dark
      )
      self.setWebviewTheme(
        theme: webviewTheme
      )
      self.setWebviewFontSize(
        fontSize: fontSize
      )
      self.setNoteDetails(note: note)
    }
  }
#endif
