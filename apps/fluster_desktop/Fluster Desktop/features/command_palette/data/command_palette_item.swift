//
//  command_palette_item.swift
//  Fluster Desktop
//
//  Created by Andrew on 1/14/26.
//

import SwiftData
import SwiftUI

public enum CommandPaletteItemType {
  case children, commandPaletteAction
}

public class CommandPaletteItem: Identifiable, Equatable {
  public let id: CommandPaletteId
  public let title: String
  public let icon: String
  public let subtitle: String?
  public let itemType: CommandPaletteItemType
  public let noneFoundText: String
  public var onAccept: (() -> Void)?
  public init(
    id: CommandPaletteId, title: String, icon: String, subtitle: String?,
    itemType: CommandPaletteItemType,
    noneFoundText: String = "No results found"
  ) {
    self.id = id
    self.title = title
    self.icon = icon
    self.subtitle = subtitle
    self.itemType = itemType
    self.noneFoundText = noneFoundText
  }

  public static func == (lhs: CommandPaletteItem, rhs: CommandPaletteItem) -> Bool {
    return lhs.id == rhs.id && lhs.title == rhs.title && lhs.icon == rhs.icon
      && lhs.subtitle == rhs.subtitle && lhs.itemType == rhs.itemType
  }
    public func children(modelContext: ModelContext, appState: AppState) -> [CommandPaletteItem] {
    fatalError("This should never be reached.")
  }
}
