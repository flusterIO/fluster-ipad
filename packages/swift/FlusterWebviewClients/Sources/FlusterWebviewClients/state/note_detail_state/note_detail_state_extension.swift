//
//  File.swift
//  FlusterWebviewClients
//
//  Created by Andrew on 3/27/26.
//

import FlusterData
import Foundation

@MainActor
extension NoteDetailState {
  public static func setNoteSummary(payload: SummaryState, eval: @escaping EvalJavascriptFunc) async throws
  {
    let action = SetNoteSummaryAction(
      type: NoteDetailActions.setNoteSummary, payload: payload)
    if let parsedData = EditorState.encodeAction(data: action) {
      try await MdxEditorClient.sendEditorStateUpdate(data: parsedData, evalulateJavaScript: eval)
    }
  }
}
