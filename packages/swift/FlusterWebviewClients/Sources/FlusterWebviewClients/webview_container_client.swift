import FlusterData
import SwiftData
import SwiftUI

public struct WebviewContainerClient {
  public enum DomClassMethod {
    case append, remove
  }
  static let webviewContainerDOMId = "webview-container"
  static func setClassByDomId(
    className: String, domId: String, method: DomClassMethod,
    evaluateJavaScript: @escaping EvalJavascriptFunc
  ) async throws {
    try await evaluateJavaScript(
      """
      document?.getElementById("\(domId)")?.classList.\(method == .append ? "add" : "remove")("\(className)"); null;
      """
    )
  }
  public static func setClassToWebviewContainer(
    className: String, method: DomClassMethod, evaluateJavaScript: @escaping EvalJavascriptFunc
  ) async throws {
    try await self.setClassByDomId(
      className: className, domId: self.webviewContainerDOMId, method: method,
      evaluateJavaScript: evaluateJavaScript)
  }
  public static func setClassToBody(
    className: String,
    method: DomClassMethod,
    evaluateJavaScript: @escaping EvalJavascriptFunc
  ) async throws {
    try await evaluateJavaScript(
      """
      document?.body?.classList.\(method == .append ? "add" : "remove")("\(className)"); null;
      """
    )
  }
  public static func setLoading(isLoading: Bool, evaluateJavaScript: @escaping EvalJavascriptFunc)
    async throws
  {
    try await self.setClassToBody(
      className: "loading",
      method: isLoading ? .append : .remove,
      evaluateJavaScript: evaluateJavaScript
    )
    if !isLoading {
      try await self.setClassToBody(
        className: "loading-main",
        method: .remove,
        evaluateJavaScript: evaluateJavaScript
      )
    }
  }
  public static func setColorScheme(
    colorScheme: ColorScheme, evaluateJavaScript: @escaping EvalJavascriptFunc
  ) async throws {
      print("Here...")
    try await evaluateJavaScript(
      """
      if (typeof window !== "undefined" && ("setDarkMode" in window)) {
          window.setDarkMode(\(colorScheme == .dark ? "true" : "false"))
      } else {
          console.log("Could not find webview container functions")
          window?.dispatchEvent(new CustomEvent("set-dark-mode", { detail: \(colorScheme == .dark ? "true" : "false") }));
          window.localStorage.setItem("dark-mode", \(colorScheme == .dark ? "\"true\"" : "\"false\""))
          document?.body?.classList?.\(colorScheme == .dark ? "add" : "remove")("dark")
      console.log("Document.body", document.body)
      }
      """)
  }
  public static func generalOnLoad(evaluateJavaScript: @escaping EvalJavascriptFunc) async throws {
    try await self.setLoading(isLoading: false, evaluateJavaScript: evaluateJavaScript)
  }
}
