//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/4/25.
//

import SwiftUI
import FlusterData

public struct DataHandlerKey: EnvironmentKey {
  public static let defaultValue: @Sendable () async -> DataHandler? = { nil }
}

public struct MainTabKey: EnvironmentKey {
  static public let defaultValue: Binding<IpadMainViewTab> = .constant(.notes)
}

extension EnvironmentValues {
  public var createDataHandler: @Sendable () async -> DataHandler? {
    get { self[DataHandlerKey.self] }
    set { self[DataHandlerKey.self] = newValue }
  }
  public var mainTab: Binding<IpadMainViewTab> {
    get { self[MainTabKey.self] }
    set { self[MainTabKey.self] = newValue }
  }
}
