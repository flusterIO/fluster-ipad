//
//  File.swift
//  FlusterWebviewClients
//
//  Created by Andrew on 3/5/26.
//

import Foundation
import UniformTypeIdentifiers
import WebKit

public class WasmSchemeHandler: NSObject, WKURLSchemeHandler {
  public func webView(_ webView: WKWebView, start urlSchemeTask: WKURLSchemeTask) {
    guard let url = urlSchemeTask.request.url else { return }

    // 1. Extract the host (the blue folder name)
    // and the path (the file inside that folder)
    let folderName = url.host ?? ""

    // url.path is "/index_mac.html" -> we need "index_mac.html"
    let filePath = url.path.trimmingCharacters(in: CharacterSet(charactersIn: "/"))

    let nsPath = filePath as NSString
    let fileExtension = nsPath.pathExtension
    let fileName = nsPath.deletingPathExtension

    // If there are subfolders inside the blue folder (e.g., assets/js/file.js),
    // we need to append the filePath's directory to the folderName.
    let internalDirectory = nsPath.deletingLastPathComponent
    let finalSubdirectory =
      internalDirectory.isEmpty ? folderName : "\(folderName)/\(internalDirectory)"

    // 2. Perform the lookup in the Main Bundle
    if let fileUrl = Bundle.main.url(
      forResource: fileName,
      withExtension: fileExtension,
      subdirectory: finalSubdirectory),
      let data = try? Data(contentsOf: fileUrl)
    {
      //      let response = URLResponse(
      //        url: url,
      //        mimeType: getMimeType(for: fileExtension),
      //        expectedContentLength: data.count,
      //        textEncodingName: nil
      //      )
      let response = HTTPURLResponse(
        url: url,
        statusCode: 200,
        httpVersion: "HTTP/1.1",
        headerFields: [
          "Content-Type": getMimeType(for: fileExtension),
          "Access-Control-Allow-Origin": "*",
          "Access-Control-Allow-Methods": "*",
          "Access-Control-Allow-Headers": "*"
        ]
      )!
      urlSchemeTask.didReceive(response)
      urlSchemeTask.didReceive(data)
      urlSchemeTask.didFinish()
      print("✅ Success: Served \(fileName).\(fileExtension) from \(finalSubdirectory)")
    } else {
      print("❌ 404: Bundle could not find file: \(fileName).\(fileExtension)")
      print("Target Subdirectory: \(finalSubdirectory)")
      urlSchemeTask.didFailWithError(NSError(domain: "WasmScheme", code: 404))
    }
  }
  public func webView(_ webView: WKWebView, stop urlSchemeTask: WKURLSchemeTask) {}

  private func getMimeType(for ext: String) -> String {
    print("Extension: \(ext)")
    switch ext.lowercased() {
      case "wasm": return "application/wasm"
      case "js", "mjs": return "application/javascript"
      case "html": return "text/html"
      case "css": return "text/css"
      default: return "application/octet-stream"
    }
  }
}
