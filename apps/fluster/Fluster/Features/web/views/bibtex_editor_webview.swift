//
//  bibtex_editor_webview.swift
//  Fluster
//
//  Created by Andrew on 11/22/25.
//

import SwiftUI
import WebKit

struct BibtexEditorWebview: UIViewRepresentable {

    let url: URL = Bundle.main.url(
        forResource: "index",
        withExtension: "html",
        subdirectory: "bibtex_editor_webview"
    )!
    @State private var webView: WKWebView = WKWebView(
        frame: .zero,
        configuration: getConfig()
    )
    @Environment(\.openURL) var openURL
    @Environment(\.modelContext) var modelContext
    @AppStorage(AppStorageKeys.webviewFontSize.rawValue) private
        var webviewFontSize: WebviewFontSize = .base
    @AppStorage(AppStorageKeys.theme.rawValue) private var theme: WebViewTheme =
        .fluster
    @State private var didSetInitialContent = false
    @State var haveSetInitialContent: Bool = false
    @AppStorage(AppStorageKeys.editorThemeDark.rawValue) private
        var editorThemeDark: CodeEditorTheme = .dracula
    @AppStorage(AppStorageKeys.editorThemeLight.rawValue) private
        var editorThemeLight: CodeEditorTheme = .githubLight
    @AppStorage(AppStorageKeys.editorKeymap.rawValue) private var editorKeymap:
        EditorKeymap = .base
    @Binding var value: String
    let container: BibtexEditorWebviewContainer
    @Environment(\.colorScheme) var colorScheme

    func makeUIView(context: Context) -> WKWebView {
        let webView = container.webView

        webView.navigationDelegate = context.coordinator
        webView.configuration.userContentController.removeScriptMessageHandler(
            forName: "bibtex-editor-update"
        )
        webView.configuration.userContentController.add(
            context.coordinator,
            name: "bibtex-editor-update"
        )
        webView.configuration.userContentController.removeScriptMessageHandler(
            forName: "bibtex-request-initial-data"
        )
        webView.configuration.userContentController.add(
            context.coordinator,
            name: "bibtex-request-initial-data"
        )
        webView.loadFileURL(url, allowingReadAccessTo: url)

        return webView
    }

    func updateUIView(_ uiView: WKWebView, context: Context) {
    }
    func makeCoordinator() -> Coordinator {
        Coordinator(self)
    }
    func setInitialProperties() {
        container.setInitialProperties(
            initialValue: value,
            codeEditorTheme: colorScheme == .dark
                ? editorThemeDark : editorThemeLight,
            editorKeymap: editorKeymap,
            theme: theme,
            fontSize: webviewFontSize,
            darkMode: colorScheme == .dark
        )
    }
}

extension BibtexEditorWebview {
    final class Coordinator: NSObject, WKNavigationDelegate,
        WKScriptMessageHandler
    {
        var parent: BibtexEditorWebview

        init(_ parent: BibtexEditorWebview) {
            self.parent = parent
        }

        func webView(_ webView: WKWebView, didFinish navigation: WKNavigation!)
        {
            guard !parent.didSetInitialContent else { return }
            parent.didSetInitialContent = true

            let body =
                parent.value
                .replacingOccurrences(of: "`", with: "\\`")

            webView.evaluateJavaScript(
                """
                window.localStorage.setItem("bibtex-editor-initial-value", `\(body)`);
                """
            )
            parent.setInitialProperties()
            parent.container.webView.isHidden = false
        }

        func userContentController(
            _ userContentController: WKUserContentController,
            didReceive message: WKScriptMessage
        ) {
            if message.name == "bibtex-editor-update",
                let str = message.body as? String
            {
                parent.value = str
            }
            if message.name == "bibtex-request-initial-data" {
                print("Request for initial editor data received...")
                parent.setInitialProperties()
                parent.container.setInitialContent(
                    entryBody: parent.value
                )
            }
        }
    }
}
