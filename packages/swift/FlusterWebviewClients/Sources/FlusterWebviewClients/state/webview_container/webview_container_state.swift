//
//  File.swift
//  FlusterWebviewClients
//
//  Created by Andrew on 3/12/26.
//

import FlusterData



@MainActor
extension WebviewContainerState {
    
    public static func setFlusterTheme(
      payload: SetFlusterThemePayload, eval: @escaping EvalJavascriptFunc
    )
      async throws
    {
      let action = SetFlusterThemeAction(
        type: WebviewContainerActions.setFlusterTheme, payload: payload)
      if let parsedData = EditorState.encodeAction(data: action) {
        try await MdxEditorClient.sendEditorStateUpdate(data: parsedData, evalulateJavaScript: eval)
      }
    }
}
