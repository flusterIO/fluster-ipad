//
//  File.swift
//  FlusterWebviewClients
//
//  Created by Andrew on 3/5/26.
//

import FlusterData
import Foundation
import UniformTypeIdentifiers
import WebKit

public class WasmSchemeHandler: NSObject, WKURLSchemeHandler {
  func parseNormalAssetUrl(url: URL, urlSchemeTask: WKURLSchemeTask) -> (
    URL, String, String, String
  )? {
    // 1. Extract path components, ignoring the leading "/"
    // Example URL: app://fluster.internal/splitview_mdx_editor_mac/index_mac.html
    // Components will be: ["splitview_mdx_editor_mac", "index_mac.html"]
    let components = url.pathComponents.filter { $0 != "/" }

    // We need at least the folder and the file name
    guard components.count >= 2 else {
      print("❌ Invalid URL structure: \(url.absoluteString)")
      urlSchemeTask.didFailWithError(NSError(domain: "WasmScheme", code: 400))
      return nil
    }

    // 2. Map the components to your Bundle structure
    let folderName = components[0]  // e.g., "splitview_mdx_editor_mac"
    let relativePath = components.dropFirst().joined(separator: "/")  // e.g., "index_mac.html"

    let nsPath = relativePath as NSString
    let fileExtension = nsPath.pathExtension
    let fileName = nsPath.deletingPathExtension

    // Handle subdirectories within the main folder (e.g., assets/js/file.js)
    let internalDirectory = nsPath.deletingLastPathComponent
    let finalSubdirectory =
      internalDirectory.isEmpty ? folderName : "\(folderName)/\(internalDirectory)"

    print("Good: \(finalSubdirectory)")

    return (
      url,
      fileName,
      fileExtension,
      finalSubdirectory
    )
  }

  func parsePublicDirAssetUrl(_ url: URL, _ urlSchemeTask: WKURLSchemeTask) -> (
    URL, String, String, String
  )? {
    print(url.absoluteString)
    do {
      let components = url.pathComponents.filter { $0 != "/" }
      let s = "app://fluster.internal/\(components.dropFirst().joined(separator: "/"))"
      print("New URL: \(s)")
      let relativePath = components.dropFirst().joined(separator: "/")
      let nsPath = relativePath as NSString
      let newUrl = Bundle.main.url(
        forResource: "MathJax_Zero", withExtension: "woff",
        subdirectory: relativePath
      )
      print(newUrl?.absoluteString)
    } catch {
      print("ERROR: Could not parse url for \(url.absoluteString)")
    }
    return nil
  }

  func parsePublicUrlString(_ url: URL, _ urlSchemeTask: WKURLSchemeTask) -> (
    URL, String, String, String
  )? {
    let components = url.pathComponents.filter { $0 != "/" }
    if components[0] == "public-protocol-hack" {
      return parsePublicDirAssetUrl(url, urlSchemeTask)
    }
    return parseNormalAssetUrl(url: url, urlSchemeTask: urlSchemeTask)
  }
  public func webView(_ webView: WKWebView, start urlSchemeTask: WKURLSchemeTask) {
    guard let _url = urlSchemeTask.request.url else { return }
    if let (url, fileName, fileExtension, subDirectory) = parsePublicUrlString(_url, urlSchemeTask)
    {
      // 3. Perform the Bundle lookup
      if let fileUrl = Bundle.main.url(
        forResource: fileName,
        withExtension: fileExtension,
        subdirectory: subDirectory),
        let data = try? Data(contentsOf: fileUrl)
      {
        let response = HTTPURLResponse(
          url: url,
          statusCode: 200,
          httpVersion: "HTTP/1.1",
          headerFields: [
            "Content-Type": getMimeType(for: fileExtension),
            "Access-Control-Allow-Origin": "*",  // Important for cross-view scripts
            "Cache-Control": "no-cache"  // Good for dev/rapid changes
          ]
        )!

        urlSchemeTask.didReceive(response)
        urlSchemeTask.didReceive(data)
        urlSchemeTask.didFinish()
        print("✅ Served: \(fileName).\(fileExtension) from \(subDirectory)")
      } else {
        print("❌ 404: \(fileName).\(fileExtension) not found in \(subDirectory)")
        urlSchemeTask.didFailWithError(NSError(domain: "WasmScheme", code: 404))
      }
    } else {
      print("❌ Error: Could not parse an asset url.")
    }
  }

  public func webView(_ webView: WKWebView, stop urlSchemeTask: WKURLSchemeTask) {}

  private func getMimeType(for ext: String) -> String {
    // WHENONLINE: Check to make sure that this woff field is right. Need to handle passing of fonts to front-end here.
    switch ext.lowercased() {
      case "wasm": return "application/wasm"
      case "js", "mjs": return "application/javascript"
      case "html": return "text/html"
      case "css": return "text/css"
      case "woff": return "text/woff"
      default: return "application/octet-stream"
    }
  }
}
