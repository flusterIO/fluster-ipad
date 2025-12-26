//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 11/29/25.
//

import Foundation
import WebKit

@MainActor
func addUserContentController(
  controller: WKUserContentController, coordinator: WKScriptMessageHandler, name: String
) {
  controller.removeScriptMessageHandler(forName: name)
  controller.add(coordinator, name: name)
}
