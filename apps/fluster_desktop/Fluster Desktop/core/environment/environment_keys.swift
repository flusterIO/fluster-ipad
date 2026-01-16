//
//  environment_keys.swift
//  Fluster Desktop
//
//  Created by Andrew on 1/15/26.
//

import SwiftUI
import Carbon
import FlusterData

public struct DataHandlerKey: EnvironmentKey {
  public static let defaultValue: @Sendable () async -> DataHandler? = { nil }
}

extension EnvironmentValues {
  public var createDataHandler: @Sendable () async -> DataHandler? {
    get { self[DataHandlerKey.self] }
    set { self[DataHandlerKey.self] = newValue }
  }
}
