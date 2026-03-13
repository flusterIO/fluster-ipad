//
//  File.swift
//  FlusterWebviewClients
//
//  Created by Andrew on 3/13/26.
//

import Foundation
import FlusterData

@MainActor
public extension DictionaryState {
    public static func setDictionaryState(payload: DictionaryState, eval: @escaping EvalJavascriptFunc) async throws {
        let action = SetDictionaryEntriesAction(
            type: DictionaryStateActions.setDictionaryEntries, payload: payload)
        if let parsedData = await EditorState.encodeAction(data: action) {
          try await MdxEditorClient.sendEditorStateUpdate(data: parsedData, evalulateJavaScript: eval)
        }
    }
}
