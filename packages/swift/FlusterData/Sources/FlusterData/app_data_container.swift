//
//  app_data_container.swift
//  Fluster
//
//  Created by Andrew on 11/27/25.
//

// import ConundrumSwift
import Foundation
import SwiftData
import ConundrumSwift
import FlusterData

@MainActor  // This was required by the mainContext key, but there's almost surely a way to do this multi-threaded. Look into this later.
@available(iOS 17, macOS 14, *)
public final class AppDataContainer {
  private static var _shared: AppDataContainer?

  // Public accessor
  public static var shared: AppDataContainer {
    guard let instance = _shared else {
      fatalError("AppDataContainer must be initialized with 'setup(constant:)' before use.")
    }
    return instance
  }

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

      if !hasLaunchedPreviously && existingNotes.isEmpty {
        let initialNotePaths = getInitialNotePaths()

        var articleWithCitations: NoteModel? = nil
        var notes: [NoteModel] = []
        for notePath in initialNotePaths {
          let url = Bundle.main.url(
            forResource: "seed\(notePath)",
            withExtension: "cdrm"
          )!
          let noteData = try Data(contentsOf: url)
          if let s = String(data: noteData, encoding: .utf8) {
            let noteModel = NoteModel.fromNoteBody(noteBody: s)
            try noteModel.preParseSync(
              modelContext: container.mainContext,
              uiParams: UiParams(
                darkMode: true, fontScalar: 1, mathFontScalar: 1.2, syntaxTheme: .dracula))
            if notePath == "/on_the_gravitational_nature_of_time" {
              articleWithCitations = noteModel
            } else {
              notes.append(noteModel)
            }
          } else {
            print("Could not decode initial note data")
          }
        }

        let initialBibliographyEntries: [BibEntryModel] = [
          BibEntryModel(
            data: """
              @article{einstein1905electrodynamics,
                  title = {On the electrodynamics of moving bodies},
                  author = {Einstein, Albert},
                  journal = {Annalen der physik},
                  volume = {17},
                  number = {10},
                  pages = {891--921},
                  year = {1905},
              }
              """,
            notes: articleWithCitations == nil ? [] : [articleWithCitations!]
          ),
          BibEntryModel(
            data: """
              @article{variableSpeedOfLightLi,
                  author = {Li, Enbang},
                  year = {2026},
                  month = {3},
                  date = {15},
                  title = {Exploring the gravito-optic effect for gravity sensing applications
                           },
                  url = {https://www.mathematicsgroup.com/articles/AMP-8-254.php},
                  doi = {https://doi.org/10.17352/amp.000154},
                  pages = {2045-2322},
                  abstract = {A cornerstone of Einstein’s special relativity is the
                              postulation that light travels at a constant speed in a vacuum
                              throughout spacetime, independent of the direction, location,
                              time, and photon energy. The constancy and universality of the
                              speed of light play an important role in modern physics for
                              understanding the universe, and their validation has been
                              demonstrated by various experiments and observations. Here we
                              present a laboratory experiment to demonstrate that photons could
                              interact with the Earth's gravitational field and consequently
                              the speed of light would be affected by the Earth's gravity. The
                              experimental results show that the speed of light increases with
                              the gravitational field, and the variation fits with a test model
                              proposed in this work. The findings of this study would open a
                              new research direction on the interaction between photons and the
                              gravitational field and could lead to exploring novel
                              gravito-optic effects and photonics-based gravity detection
                              technologies.},
              }
              """,
            notes: articleWithCitations == nil ? [] : [articleWithCitations!]
          ),
          BibEntryModel(
            data: """
              @article{Michelson_Morely,
                  author = {{Albert Abraham Michelson, Edward Morley}},
                  journal = {{American Journal of Science}},
                  number = {203},
                  volume = {34},
                  pages = {333-345},
                  title = {{On the Relative Motion of the Earth and the Luminiferous Ether}},
                  year = {1887},
              }
              """,
            notes: articleWithCitations == nil ? [] : [articleWithCitations!]
          ),
          BibEntryModel(
            data: """
              @article{gordon_determining_2008,
                  title = {Determining the motion of the {Solar} system relative to the cosmic
                           microwave background using {Type} {Ia} supernovae},
                  volume = {387},
                  issn = {0035-8711},
                  url = {https://doi.org/10.1111/j.1365-2966.2008.13239.x},
                  doi = {10.1111/j.1365-2966.2008.13239.x},
                  abstract = {We estimate the Solar system motion relative to the cosmic
                              microwave background using Type Ia supernovae (SNe) measurements.
                              We take into account the correlations in the error bars of the
                              SNe measurements arising from correlated peculiar velocities.
                              Without accounting for correlations in the peculiar velocities,
                              the SNe data we use appear to detect the peculiar velocity of the
                              Solar system at about the 3.5σ level. However, when the
                              correlations are correctly accounted for, the SNe data only
                              detect the Solar system peculiar velocity at about the 2.5σ
                              level. We forecast that the Solar system peculiar velocity will
                              be detected at the 9σ level by GAIA and the 11σ level by the
                              Large Synoptic Survey Telescope. For these surveys, we find the
                              correlations are much less important as most of the signal comes
                              from higher redshifts where the number density of SNe is
                              insufficient for the correlations to be important.},
                  number = {1},
                  urldate = {2022-10-19},
                  journal = {Monthly Notices of the Royal Astronomical Society},
                  author = {Gordon, Christopher and Land, Kate and Slosar, Anže},
                  month = {jun},
                  year = {2008},
                  pages = {371--376},
              } 
              """,
            notes: articleWithCitations == nil ? [] : [articleWithCitations!]
          ),
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
              """, notes: articleWithCitations == nil ? [] : [articleWithCitations!])
        ]
        for bibEntry in initialBibliographyEntries {
          container.mainContext.insert(bibEntry)
        }
        for note in notes {
          container.mainContext.insert(note)
        }
        if articleWithCitations != nil {
          for entry in initialBibliographyEntries {
            articleWithCitations!.addCitation(citation: entry, strategy: .userAdded)
          }
          container.mainContext.insert(articleWithCitations!)
        } else {
          print("Error: Could not find the intended article to demonstrate citations.")
        }
        try container.mainContext.save()
        UserDefaults.standard.set(true, forKey: AppStorageKeys.hasLaunchedPreviously.rawValue)
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
  public static func setup() {
    guard _shared == nil else { return }  // Prevent re-initialization
    _shared = AppDataContainer()
  }
}
