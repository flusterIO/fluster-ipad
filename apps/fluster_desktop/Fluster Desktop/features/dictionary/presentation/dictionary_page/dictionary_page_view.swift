//
//  dictionary_page_view.swift
//  Fluster
//
//  Created by Andrew on 2/23/26.
//

import FlatBuffers
import FlusterData
import SwiftData
import SwiftUI
import WebKit

struct DictionaryPageView: View {
  @State private var webview: WKWebView = WKWebView(
    frame: .infinite, configuration: getWebViewConfig()
  )
  @Query(sort: \DictionaryEntryModel.label, order: .forward) private var entries: [DictionaryEntryModel]
  var body: some View {
    WebViewContainerView(
      webview: $webview,
      url: Bundle.main.url(
        forResource: "index",
        withExtension: "html",
        subdirectory: "dictionary_webview"
      )!,
      messageHandlerKeys: [
        MdxPreviewWebviewActions.onTagClick.rawValue,
        SplitviewEditorWebviewActions.setWebviewLoaded.rawValue,
        SplitviewEditorWebviewActions.requestSplitviewEditorData.rawValue,
        DictionaryWebviewActions.requestDictionaryData.rawValue,
        MdxPreviewWebviewActions.requestNoteData.rawValue
      ],
      messageHandler: messageHandler,
      onLoad: onWebviewLoad
    )
  }

  func setDictionaryContent(entries: [DictionaryEntryModel]) async throws {
    var builder = FlatBufferBuilder()
    let entries_offset: [Offset] = entries.map({ dict in
      let idOffset = builder.create(string: dict.id)
      let labelOffset = builder.create(string: dict.label)
      let bodyOffset = builder.create(string: dict.body)
      let entry_offset = Dictionary_DictionaryEntryResultBuffer.createDictionaryEntryResultBuffer(
        &builder, labelOffset: labelOffset, bodyOffset: bodyOffset)
      return entry_offset
    })
    let vector_offset = builder.createVector(ofOffsets: entries_offset)
    let data = Dictionary_DictionaryData.createDictionaryData(
      &builder, entriesVectorOffset: vector_offset)
    builder.finish(offset: data)
    let bytes: [UInt8] = Array(builder.data)
    Task {
      do {
        try await webview.evaluateJavaScript(
          """
          window.localStorage.setItem("\(DictionaryWebviewStorageKeys.dictionaryData.rawValue)", `\(bytes)`);
          window.dispatchEvent(new CustomEvent("\(DictionaryWebviewEvents.setDictionaryData.rawValue)", {
              detail: \(bytes)
          }))
          """
        )
      } catch {
        print("Error: \(error.localizedDescription)")
      }
    }
  }

  func onWebviewLoad() async {
    Task {
      do {
        try await setDictionaryContent(entries: entries)
        print("Loaded initial editor data")
      } catch {
        print("Error initalizing Mdx Editor Webview: \(error.localizedDescription)")
      }
    }
  }
    
  public func messageHandler(_ handlerKey: String, _ messageBody: Any) {
    switch handlerKey {
      case MdxPreviewWebviewActions.requestNoteData.rawValue:
        Task(priority: .high) {
          await self.onWebviewLoad()
        }
      case DictionaryWebviewActions.requestDictionaryData.rawValue:
        Task(priority: .high) {
          await self.onWebviewLoad()
        }
      default:
        print("No action implemented for \(handlerKey)")
        return
    }
  }
    
}
