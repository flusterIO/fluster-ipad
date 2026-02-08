//
//  add_user_content_controller.swift
//  Fluster
//
//  Created by Andrew on 2/8/26.
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

@MainActor
func addContentControllerList(items: [String], controller: WKUserContentController, coordinator: WKScriptMessageHandler) {
    for item in items {
        addUserContentController(controller: controller, coordinator: coordinator, name: item)
    }
}
