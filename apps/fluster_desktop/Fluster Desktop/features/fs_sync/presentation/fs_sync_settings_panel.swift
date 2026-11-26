//
//  fs_sync_settings_panel.swift
//  Fluster Desktop
//

import FlusterData
import SwiftData
import SwiftUI

/// "Sync now", sync policies, and the last sync's result. Lives in the Notes settings section.
struct FsSyncSettingsPanel: View {
  @Environment(FsSyncController.self) private var controller: FsSyncController?
  @AppStorage(AppStorageKeys.notesDirectory.rawValue) private var notesDirectory: String = ""
  @AppStorage(FsSyncStorageKeys.watchForChanges.rawValue) private var watchForChanges: Bool = false
  @AppStorage(FsSyncStorageKeys.conflictPolicy.rawValue) private var conflictPolicy:
    FsSyncConflictPolicy = .preferNewest
  @AppStorage(FsSyncStorageKeys.missingFilePolicy.rawValue) private var missingFilePolicy:
    FsSyncMissingFilePolicy = .keepNote

  var body: some View {
    if let controller {
      VStack(alignment: .leading, spacing: 16) {
        Divider()
        Text("Synchronization")
          .font(.headline)
        syncControls(controller)
        Toggle("Sync automatically when files change", isOn: $watchForChanges)
          .toggleStyle(.switch)
          .onChange(of: watchForChanges) { _, enabled in
            if enabled {
              controller.startWatching()
              controller.syncNow()
            } else {
              controller.stopWatching()
            }
          }
          .onChange(of: notesDirectory) { _, _ in
            if watchForChanges { controller.startWatching() }
          }
        statusView(controller)
        policyPickers
      }
    }
  }

  @ViewBuilder
  private func syncControls(_ controller: FsSyncController) -> some View {
    HStack(spacing: 12) {
      Button(
        action: { controller.syncNow() },
        label: { Label("Sync Now", systemImage: "arrow.triangle.2.circlepath") }
      )
      .buttonStyle(.borderedProminent)
      .disabled(controller.isRunning || notesDirectory.isEmpty)
      if controller.isRunning {
        Button("Cancel") { controller.cancel() }
      }
    }
    Text(
      "Imports new and changed files from your notes directory into Fluster through the Conundrum server. Files are only read, never modified."
    )
    .font(.caption)
    .foregroundStyle(.secondary)
  }

  @ViewBuilder
  private func statusView(_ controller: FsSyncController) -> some View {
    switch controller.status {
      case .idle:
        if let report = controller.lastReport {
          FsSyncReportView(report: report)
        }
      case .running(let progress):
        VStack(alignment: .leading, spacing: 6) {
          if let fraction = progress.fractionCompleted {
            ProgressView(value: fraction)
          } else {
            ProgressView().progressViewStyle(.linear)
          }
          Text(progress.description)
            .font(.caption)
            .foregroundStyle(.secondary)
        }
      case .finished(let report):
        FsSyncReportView(report: report)
      case .failed(let message):
        Label(message, systemImage: "exclamationmark.triangle.fill")
          .foregroundStyle(.red)
          .font(.callout)
    }
  }

  private var policyPickers: some View {
    VStack(alignment: .leading, spacing: 12) {
      VStack(alignment: .leading) {
        Picker("When a note changed in both places", selection: $conflictPolicy) {
          ForEach(FsSyncConflictPolicy.allCases) { policy in
            Text(policy.label).tag(policy)
          }
        }
        Text(
          "\"Newest wins\" takes the file only if it was saved after the note was last changed in Fluster."
        )
        .font(.caption)
        .foregroundStyle(.secondary)
      }
      VStack(alignment: .leading) {
        Picker("When a synced file is deleted", selection: $missingFilePolicy) {
          ForEach(FsSyncMissingFilePolicy.allCases) { policy in
            Text(policy.label).tag(policy)
          }
        }
        Text(
          "Files hidden by .gitignore count as deleted. If no note files are found at all, nothing is unlinked or deleted."
        )
        .font(.caption)
        .foregroundStyle(.secondary)
      }
    }
  }
}

/// Summary of a finished sync, with any per-file problems listed underneath.
struct FsSyncReportView: View {
  let report: FsSyncReport
  private static let maxIssuesShown = 50

  var body: some View {
    VStack(alignment: .leading, spacing: 6) {
      Label(
        report.summary,
        systemImage: report.failures.isEmpty ? "checkmark.circle.fill" : "exclamationmark.circle.fill"
      )
      .foregroundStyle(report.failures.isEmpty ? Color.green : Color.orange)
      .font(.callout)
      Text("Finished \(report.finishedAt.formatted(.relative(presentation: .named)))")
        .font(.caption)
        .foregroundStyle(.secondary)
      issueGroup("Conflicts", report.conflicts)
      issueGroup("Failures", report.failures)
      issueGroup("Warnings", report.warnings)
    }
  }

  @ViewBuilder
  private func issueGroup(_ title: String, _ issues: [FsSyncIssue]) -> some View {
    if !issues.isEmpty {
      DisclosureGroup("\(title) (\(issues.count))") {
        VStack(alignment: .leading, spacing: 4) {
          ForEach(issues.prefix(Self.maxIssuesShown)) { issue in
            VStack(alignment: .leading, spacing: 0) {
              Text(issue.relativePath).font(.caption.monospaced())
              Text(issue.message).font(.caption).foregroundStyle(.secondary)
            }
          }
          if issues.count > Self.maxIssuesShown {
            Text("…and \(issues.count - Self.maxIssuesShown) more")
              .font(.caption)
              .foregroundStyle(.secondary)
          }
        }
        .frame(maxWidth: .infinity, alignment: .leading)
        .textSelection(.enabled)
      }
    }
  }
}

#Preview {
  FsSyncSettingsPanel()
    .environment(FsSyncController(modelContainer: AppDataContainer.shared.sharedModelContainer))
    .padding()
}
