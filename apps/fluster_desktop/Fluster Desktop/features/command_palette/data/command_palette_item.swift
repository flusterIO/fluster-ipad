//
//  command_palette_item.swift
//  Fluster Desktop
//
//  Created by Andrew on 1/14/26.
//

import Foundation
import SwiftData

public class CommandPaletteItem: Identifiable, Equatable {
  public let id: CommandPaletteId
  public let title: String
  public let icon: String
  public let subtitle: String?
  public let hasChildren: Bool
  public init(
    id: CommandPaletteId, title: String, icon: String, subtitle: String?, hasChildren: Bool = false
  ) {
    self.id = id
    self.title = title
    self.icon = icon
    self.subtitle = subtitle
    self.hasChildren = hasChildren
  }

  public static func == (lhs: CommandPaletteItem, rhs: CommandPaletteItem) -> Bool {
    return lhs.id == rhs.id && lhs.title == rhs.title && lhs.icon == rhs.icon
      && lhs.subtitle == rhs.subtitle && lhs.hasChildren == rhs.hasChildren
  }
    public func children(modelContext: ModelContext) -> [CommandPaletteItem] {
    fatalError("This should never be reached.")
  }
}
