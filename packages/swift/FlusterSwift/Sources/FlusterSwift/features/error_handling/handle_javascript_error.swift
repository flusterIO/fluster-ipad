//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/8/25.
//

import FlatBuffers
import Foundation
import FlusterData

public func handleJavascriptError(base64String: String) {
  if var data = Data(base64Encoded: base64String) {
    var buf = ByteBuffer(data: data)
    let e: SharedWebviewData_WebviewJavascriptError = try! getCheckedRoot(byteBuffer: &buf)
    print("Javascript Error Message: \(e.message ?? "Javascript error message not found")")
    print("Javascript Error: \(e.error ?? "Javascript error not found")")
  } else {
    print("Error: Could not deserialize javascript error")
  }
}
