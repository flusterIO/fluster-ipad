//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 3/24/26.
//

import FlusterData
import SwiftUI

public enum NoteSummaryGenerationStrategy: String, CaseIterable {
  /// Runs nightly if the note's utime > summary's ctime.
  case onChange = "Nightly"
  case ifNoSummaryExists = "If no summary exists"
  case manual = "Manual"
}

public struct NoteSummaryGenerationMethodInput: View {
  @AppStorage(AppStorageKeys.noteSummaryGenerationStrategy.rawValue) private var strategy:
    NoteSummaryGenerationStrategy = .ifNoSummaryExists
  public init() {
  }
  public var body: some View {
    Picker(
      selection: $strategy,
      content: {
        ForEach(NoteSummaryGenerationStrategy.allCases, id: \.rawValue) { cit in
          Text(cit.rawValue).tag(cit)
        }
      },
      label: {
        Label(
          title: {
            Text("Summary Strategy")
          },
          icon: {
            Image(systemName: "externaldrive.badge.questionmark.ar")
          })
      })
  }
}

#Preview {
  NoteSummaryGenerationMethodInput()
}
