//
//  File.swift
//  FlusterWebviewClients
//
//  Created by Andrew on 4/4/26.
//

import Foundation
import FlusterData
import ConundrumSwift

public extension ConundrumState {
    public static func setConundrumError(error: ConundrumError, eval: @escaping EvalJavascriptFunc) async throws {
        let action = SetConundrumErrorsStateAction(
            type: ConundrumStateActions.setConundrumErrors, payload: [error])
        if let parsedData = await EditorState.encodeAction(data: action) {
          try await MdxEditorClient.sendEditorStateUpdate(data: parsedData, evalulateJavaScript: eval)
        }
    }
}
