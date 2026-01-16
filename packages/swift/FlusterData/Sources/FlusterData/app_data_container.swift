//
//  app_data_container.swift
//  Fluster
//
//  Created by Andrew on 11/27/25.
//

import FlusterSwiftMdxParser
import Foundation
import SwiftData

public let INITIAL_NOTES_DATA_PATH = "initial_note_docs/initial_notes_parsed_data"

@MainActor  // This was required by the mainContext key, but there's almost surely a way to do this multi-threaded. Look into this later.
@available(iOS 17, macOS 14, *)
public final class AppDataContainer {
  public static let shared = AppDataContainer()
  public var sharedModelContainer: ModelContainer {
    let hasLaunchedPreviously = UserDefaults.standard.bool(
      forKey: AppStorageKeys.hasLaunchedPreviously.rawValue
    )
    let schema = Schema([
      NoteModel.self,
      BibEntryModel.self
    ])
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

      if !hasLaunchedPreviously && existingNotes.isEmpty {
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
              """),
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
              """),
            BibEntryModel(
            data: """
                @letter{Einstein_Besso_Letter,
                author = {Einstein, Albert},
                year = {1955},
                month = {March},
                title = {{Letter to the family Michele Besso following his death}},
                }
                """)
        ]
        for bibEntry in initialBibliographyEntries {
          container.mainContext.insert(bibEntry)
        }
          try container.mainContext.save()

        let noteData = InitialNotesDataJsonDecoder.decode(
          from: INITIAL_NOTES_DATA_PATH
        )

        if let nd = noteData {
          nd.forEach { noteItemData in
            let noteModel = NoteModel.fromInitialDataParsingResult(
              data: noteItemData,
              existingTags: [],
              existingCitations: initialBibliographyEntries,
              forceUserDefinedCitations: noteItemData.noteId == "onTheGravitationalNatureOfTime"
            )
            container.mainContext.insert(noteModel)
          }
        } else {
          fatalError("Failed to load initial notes.")
        }
      }
      try container.mainContext.save()
      return container
    } catch {
      fatalError("Could not create ModelContainer: \(error)")
    }
  }

  private init() {}

  public func dataHandlerCreator() -> @Sendable () async -> DataHandler {
    let container = sharedModelContainer
    return {
      DataHandler(modelContainer: container)
    }
  }
}

