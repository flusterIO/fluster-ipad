//
//  environment_keys.swift
//  Fluster Desktop
//
//  Created by Andrew on 1/15/26.
//

import Carbon
import FlusterData
import SwiftUI

public struct DataHandlerKey: EnvironmentKey {
  public static let defaultValue: @Sendable () async -> DataHandler? = { nil }
}

struct MainViewEnvKey: EnvironmentKey {
    static let defaultValue: Binding<MainViewKey>? = nil
}


extension EnvironmentValues {
  public var createDataHandler: @Sendable () async -> DataHandler? {
    get { self[DataHandlerKey.self] }
    set { self[DataHandlerKey.self] = newValue }
  }
  public var mainViewKey: Binding<MainViewKey>? {
    get { self[MainViewEnvKey.self] }
    set { self[MainViewEnvKey.self] = newValue }
  }
}
