//
//  FlusterApp.swift
//  Fluster
//
//  Created by Andrew on 10/28/25.
//

import FlusterData
import FlusterSwift
import SwiftData
import SwiftUI
import UniformTypeIdentifiers
import WebKit

// typealias NoteModel = AppSchemaV1.NoteModel
// typealias BibEntryModel = AppSchemaV1.BibEntryModel
// typealias SubjectModel = AppSchemaV1.SubjectModel
// typealias TagModel = AppSchemaV1.TagModel
// typealias TopicModel = AppSchemaV1.TopicModel
// typealias MarkdownNote = AppSchemaV1.MarkdownNote

@main
struct FlusterApp: App {
  let appData: AppDataContainer
  @AppStorage(AppStorageKeys.hasLaunchedPreviously.rawValue) private var hasPreviouslyLaunched:
    Bool = false

  init() {
    SchemeRegistration.registerCustomScheme()
    AppDataContainer.setup(
      initialNotesUrl: Bundle.main.url(
        forResource: "initial_note_docs/initial_notes_parsed_data",
        withExtension: "json"
      )!
    )
    self.appData = AppDataContainer.shared
  }

  var body: some Scene {
    WindowGroup {
      MainView()
        .ignoresSafeArea()
        .environment(\.createDataHandler, appData.dataHandlerCreator())
    }
    .modelContainer(appData.sharedModelContainer)
  }
}

struct SchemeRegistration {
  static func registerCustomScheme() {
    let register = NSSelectorFromString("registerSchemeForCustomProtocol:")

    // We cast to NSObjectProtocol to check for the selector safely
    if let webViewClass = WKWebView.self as Any as? NSObjectProtocol,
      webViewClass.responds(to: register)
    {
      // Registering "app" as a custom protocol makes WebKit treat it
      // like "file://" or "https://", enabling persistent IndexedDB.
      webViewClass.perform(register, with: "app")

      print("🚀 Registered 'app://' as a secure custom scheme.")
    }
  }
}
