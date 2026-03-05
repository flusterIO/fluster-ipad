//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 3/4/26.
//

import FlusterData
import FlusterWebviewClients
import Foundation

public typealias EvalJavascriptFunc = @Sendable (String) async throws -> Sendable?




/// Takes common Swift language objects, creates an `AnyCrossLanguageEditorAction` and sends it to Typescript.
public class EditorStateManager {
  public init() {
  }
  static func encodeData(data: Encodable) -> String? {
    let encoder = JSONEncoder()
    #if DEBUG
      encoder.outputFormatting = .prettyPrinted
    #endif

    do {
      let jsonData = try encoder.encode(data)
      let jsonString = String(data: jsonData, encoding: .utf8)
      return jsonString
    } catch {
      print("Error encoding user: \(error.localizedDescription)")
    }
    return nil
  }
  public static func setEditorSaveMethod(
    saveMethod: EditorSaveMethod, eval: @escaping EvalJavascriptFunc
  ) async throws {
    let data = SetEditorSaveMethodEditorAction(type: .setEditorSaveMethod, payload: saveMethod)
    if let parsedData = EditorStateManager.encodeData(data: saveMethod) {
      try await MdxEditorClient.sendEditorStateUpdate(data: parsedData, evalulateJavaScript: eval)
    }
  }
}
