//
//  app_container.swift
//  Fluster Desktop
//
//  Created by Andrew on 1/15/26.
//

import ConundrumSwift
import FlusterData
import Foundation
import SwiftData

@MainActor  // This was required by the mainContext key, but there's almost surely a way to do this multi-threaded. Look into this later.
@available(iOS 26, macOS 14, *)
public final class AppDataContainer {
  // private let initialNotesUrl: URL
  public static let shared = AppDataContainer()
  public var sharedModelContainer: ModelContainer {
    let hasLaunchedPreviously = UserDefaults.standard.bool(
      forKey: AppStorageKeys.hasLaunchedPreviously.rawValue
    )
    let schema = Schema(AppSchemaV1.models)
    let modelConfiguration = ModelConfiguration(
      schema: schema,
      isStoredInMemoryOnly: false
    )
    do {
      let container = try ModelContainer(
        for: schema,
        migrationPlan: AppDataMigrationPlan.self,
        configurations: [modelConfiguration]
      )

      let fetchDescriptor = FetchDescriptor<NoteModel>()

      let existingNotes = try container.mainContext.fetch(fetchDescriptor)
        
print("Here 1: \(hasLaunchedPreviously)")
      if !hasLaunchedPreviously && existingNotes.isEmpty {
          print("Here?")
        let initialBibliographyEntries: [BibEntryModel] = [
          BibEntryModel(
            data: """
              @article{IvesStillwell,
              author = {Herbert E. Ives and G. R. Stilwell},
              journal = {J. Opt. Soc. Am.},
              keywords = {Doppler effect; Hydrogen; Light sources; Mirrors; Optical clocks; Transverse effects},
              number = {7},
              pages = {215--226},
              publisher = {Optica Publishing Group},
              title = {An Experimental Study of the Rate of a Moving Atomic Clock},
              volume = {28},
              month = {Jul},
              year = {1938},
              url = {https://opg.optica.org/abstract.cfm?URI=josa-28-7-215},
              doi = {10.1364/JOSA.28.000215},
              abstract = {},
              }                
              """, notes: []),
          BibEntryModel(
            data: """
              @article{PoundRebka,
                title = {Apparent Weight of Photons},
                author = {Pound, R. V. and Rebka, G. A.},
                journal = {Phys. Rev. Lett.},
                volume = {4},
                issue = {7},
                pages = {337--341},
                numpages = {0},
                year = {1960},
                month = {Apr},
                publisher = {American Physical Society},
                doi = {10.1103/PhysRevLett.4.337},
                url = {https://link.aps.org/doi/10.1103/PhysRevLett.4.337}
              }
              """, notes: []),
          BibEntryModel(
            data: """
              @letter{Einstein_Besso_Letter,
              author = {Einstein, Albert},
              year = {1955},
              month = {March},
              title = {{Letter to the family Michele Besso following his death}},
              }
              """, notes: [])
        ]
        for bibEntry in initialBibliographyEntries {
          container.mainContext.insert(bibEntry)
        }
        try container.mainContext.save()

        let initialNotePaths = getInitialNotePaths()

        for notePath in initialNotePaths {
          let url = Bundle.main.url(
            forResource: "seed\(notePath)",
            withExtension: "cdrm"
          )!
          let noteData = try Data(contentsOf: url)
          if let s = String(data: noteData, encoding: .utf8) {
              print("Note Content: \(s)")
            let noteModel = NoteModel.fromNoteBody(noteBody: s)
            Task {
              try await noteModel.preParse(
                modelContext: container.mainContext,
                uiParams: UiParams(
                  darkMode: true, fontScalar: 1, mathFontScalar: 1.2, syntaxTheme: .dracula))
              container.mainContext.insert(noteModel)
            }
          } else {
            print("Could not decode initial note data")
          }
        }
        try container.mainContext.save()
//          UserDefaults.standard.set(true, forKey: AppStorageKeys.hasLaunchedPreviously.rawValue)
      }
      return container
    } catch {
      fatalError("Could not create ModelContainer: \(error)")
    }
  }

  private init() {
  }

  public func dataHandlerCreator() -> @Sendable () async -> DataHandler {
    let container = sharedModelContainer
    return {
      DataHandler(modelContainer: container)
    }
  }
}
