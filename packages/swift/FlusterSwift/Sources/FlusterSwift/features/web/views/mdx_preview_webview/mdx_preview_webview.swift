//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 11/29/25.
//

import SwiftData
import SwiftUI
import WebKit
//import FlusterS

public struct MdxPreviewWebview: UIViewRepresentable {

    @State private var webView: WKWebView = WKWebView(
        frame: .zero,
        configuration: getConfig()
    )
    @State private var didSetInitialContent = false
    @State var haveSetInitialContent: Bool = false
    @Environment(\.openURL) var openURL
    @Environment(\.modelContext) var modelContext
    @Environment(\.colorScheme) var colorScheme
    @AppStorage(AppStorageKeys.webviewFontSize.rawValue) private
        var webviewFontSize: WebviewFontSize = .base
    let url: URL
    @Binding var theme: WebViewTheme
    @Binding var editorThemeDark: CodeSyntaxTheme
    @Binding var editorThemeLight: CodeSyntaxTheme
    @Binding var editingNote: NoteModel?
    @Binding var editorKeymap: EditorKeymap
    @Binding var shouldShowEditor: Bool
    @Binding var viewportHeight: CGFloat

    let container: MdxPreviewWebviewContainer

    public init(
        url: URL,
        theme: Binding<WebViewTheme>,
        editorThemeDark: Binding<CodeSyntaxTheme>,
        editorThemeLight: Binding<CodeSyntaxTheme>,
        editingNote: Binding<NoteModel?>,
        editorKeymap: Binding<EditorKeymap>,
        shouldShowEditor: Binding<Bool>,
        viewportHeight: Binding<CGFloat>,
        container: MdxPreviewWebviewContainer,
    ) {
        self.url = url
        self._theme = theme
        self._editorThemeDark = editorThemeDark
        self._editorThemeLight = editorThemeLight
        self._editingNote = editingNote
        self._editorKeymap = editorKeymap
        self._shouldShowEditor = shouldShowEditor
        self._viewportHeight = viewportHeight
        self.container = container
    }

    public func makeUIView(context: Context) -> WKWebView {
        let webView = container.webView

        webView.navigationDelegate = context.coordinator
        let controllers = [
            SplitviewEditorWebviewActions.requestParsedMdxContent.rawValue,
            SplitviewEditorWebviewActions.setIsLandscape.rawValue,
            SplitviewEditorWebviewActions.onTagClick.rawValue,
            SplitviewEditorWebviewActions.setWebviewLoaded.rawValue,

        ]

        for controllerName in controllers {
            addUserContentController(
                controller: webView.configuration.userContentController,
                coordinator: context.coordinator,
                name: controllerName
            )
        }
        // Loading the page only once
        webView.loadFileURL(url, allowingReadAccessTo: url)

        return webView
    }

    public func updateUIView(_ uiView: WKWebView, context: Context) {
        //        uiView.scrollView.contentInset = .zero
        //        uiView.scrollView.scrollIndicatorInsets = .zero
    }
    public func makeCoordinator() -> Coordinator {
        Coordinator(self)
    }
    public func setInitialProperties() {
        container.setInitialProperties(
            editingNote: editingNote,
            codeEditorTheme: colorScheme == .dark
                ? editorThemeDark : editorThemeLight,
            editorKeymap: editorKeymap,
            theme: theme,
            fontSize: webviewFontSize,
            editorThemeDark: editorThemeDark,
            editorThemeLight: editorThemeLight,
            darkMode: colorScheme == .dark
        )
    }
    public func setInitialContent() {
        if editingNote != nil {
            container.setInitialContent(note: editingNote!)
        } else {
            container.runJavascript(
                """
                window.localStorage.setItem("\(SplitviewEditorWebviewLocalStorageKeys.parsedMdxData.rawValue)", "")
                window.setEditorContent("")
                """
            )
        }
    }
    
    public func handleTagClick(tagValue: String) {
        // TODO: Handle tag click here by navigating to proper tag search results. Create a full screen cover that will open search results that will then be dismissed on click.
    }
}

extension MdxPreviewWebview {
    public final class Coordinator: @MainActor NSObject, WKNavigationDelegate,
        WKScriptMessageHandler
    {
        var parent: MdxPreviewWebview
        

        init(_ parent: MdxPreviewWebview) {
            self.parent = parent
        }

        public func webView(
            _ webView: WKWebView,
            didFinish navigation: WKNavigation!
        ) {
            guard !parent.didSetInitialContent else { return }
            parent.didSetInitialContent = true

            if let preParsedNoteBody = self.parent.editingNote?.markdown.preParsedBody {
                webView.evaluateJavaScript(
                    """
                    window.localStorage.setItem("\(SplitviewEditorWebviewLocalStorageKeys.parsedMdxData.rawValue)", \(preParsedNoteBody.toQuotedJavascriptString()));
                    """
                )
            }
            parent.setInitialProperties()
            parent.container.webView.isHidden = false
        }

        public func userContentController(
            _ userContentController: WKUserContentController,
            didReceive message: WKScriptMessage
        ) {
            switch message.name {
            case SplitviewEditorWebviewActions.onTagClick.rawValue:
                parent.handleTagClick(tagValue: message.body as! String)
//            case "set-preview-viewport-height":
//                if let n = NumberFormatter().number(
//                    from: message.body as! String
//                ) {
//                    parent.viewportHeight = CGFloat(truncating: n)
//                } else {
//                    parent.viewportHeight = UIScreen.main.bounds.height
//                }
            case SplitviewEditorWebviewActions.setIsLandscape.rawValue:
                parent.shouldShowEditor = (message.body as! String) == "true"
            case SplitviewEditorWebviewActions.requestParsedMdxContent.rawValue:
                parent.setInitialProperties()
                parent.setInitialContent()
            default:
                return
            }
        }
    }
}
