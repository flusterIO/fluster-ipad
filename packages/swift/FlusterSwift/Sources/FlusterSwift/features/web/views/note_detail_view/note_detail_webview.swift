//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/5/25.
//

import SwiftUI
import WebKit


public struct NoteDetailWebview: UIViewRepresentable {
    @State private var show: Bool = false
    public let url: URL = Bundle.main.url(
        forResource: "index",
        withExtension: "html",
        subdirectory: "note_detail_webview"
    )!
    
    public let note: NoteModel
    
    public let container: NoteDetailWebviewContainer

    public init(note: NoteModel) {
        self.note = note
        self.container = NoteDetailWebviewContainer()
    }
    
    public func makeUIView(context: Context) -> WKWebView {
        let webView = container.webView

        webView.navigationDelegate = context.coordinator
        webView.isHidden = true
        let editorContentControllers = [
            NoteDetailWebviewActions.requestNoteDetailData.rawValue,
            NoteDetailWebviewActions.setWebviewLoaded.rawValue,
        ]
        for controllerName in editorContentControllers {
            addUserContentController(
                controller: webView.configuration.userContentController,
                coordinator: context.coordinator,
                name: controllerName
            )
        }

        webView.loadFileURL(url, allowingReadAccessTo: url)

        return webView
    }

    public func updateUIView(_ uiView: WKWebView, context: Context) {
        uiView.isHidden = !show
    }
    public func makeCoordinator() -> Coordinator {
        Coordinator(self)
    }
}

@MainActor
public extension NoteDetailWebview {
    public final class Coordinator: NSObject, WKNavigationDelegate,
        WKScriptMessageHandler
    {
        var parent: NoteDetailWebview

        public init(_ parent: NoteDetailWebview) {
            self.parent = parent
        }

        public func webView(
            _ webView: WKWebView,
            didFinish navigation: WKNavigation!
        ) {
            // On Load
            parent.container.webView.isHidden = false
        }
        
        public func webView(
            _ webView: WKWebView,
            didFail navigation: WKNavigation!,
            withError error: Error
        ) {
            print(
                "WebView navigation failed with error: \(error.localizedDescription)"
            )
        }
        @MainActor
        public func userContentController(
            _ userContentController: WKUserContentController,
            didReceive message: WKScriptMessage
        ) {
            switch message.name {
            case NoteDetailWebviewActions.setWebviewLoaded.rawValue:
                self.parent.show = true
            case NoteDetailWebviewActions.requestNoteDetailData.rawValue:
                self.parent.container.setNoteDetails(note: parent.note)
            default:
                return
            }
        }
    }
}
