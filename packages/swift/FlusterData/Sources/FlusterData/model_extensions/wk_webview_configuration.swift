//
//  File.swift
//  FlusterData
//
//  Created by Andrew on 3/9/26.
//

import Foundation
import WebKit

@MainActor
public class WebContextManager {
  static let shared = WebContextManager()
  // This is the "brain" that syncs the storage
  let sharedProcessPool = WKProcessPool()
  let dataStore = WKWebsiteDataStore.default()
  private init() {}

  public static func createSharedConfiguration() -> WKWebViewConfiguration {
    let config = WKWebViewConfiguration()
    config.processPool = WebContextManager.shared.sharedProcessPool
    config.websiteDataStore = WebContextManager.shared.dataStore
    return config
  }
}
