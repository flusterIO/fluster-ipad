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
    // This is the "brain" that syncs the storage
    public static let sharedProcessPool = WKProcessPool()
    
    private init() {}
    
    public static func createSharedConfiguration() -> WKWebViewConfiguration {
        let config = WKWebViewConfiguration()
        config.processPool = WebContextManager.sharedProcessPool
        // Use the default persistent data store (LocalStorage/Cookies)
        config.websiteDataStore = .default()
        return config
    }
}
