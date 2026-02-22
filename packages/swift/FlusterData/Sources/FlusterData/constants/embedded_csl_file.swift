//
//  File.swift
//  FlusterData
//
//  Created by Andrew on 2/21/26.
//

import Foundation

/// A wrapper around the EmbeddedCslFile enum provided from rust. The same exact values with some utility methods and some parent classes.
public enum EmbeddedCslFileSwift: String, CaseIterable, Hashable {
  case americanChemicalSociety = "American Chemical Society"
  case americanInstituteOfPhysics = "American Institute of Physics"
  case americanMedicalAssociation = "American Medical Association"
  case americanPoliticalScienceAssociation = "American Political Science Association"
  case americanSociologicalAssociation = "American Sociological Association"
  case apa = "APA"
  case associationForComputingMachinery = "Association for Computing Machinery"
  case cell = "Cell"
  case chicagoAuthorDate = "Chicago: Author, Date"
  case chicagoNotesBibliography = "Chicago: Notes bibliography"
  case harvardCiteThemRight = "Harvard: Cite them right"
  case ieee = "Ieee"
  case modernLanguageAssociation = "Modern Language Association"
  case nature = "Nature"
  case science = "Science"
  case springerVancouver = "Springer: Vancouver"

  func toEmbeddedFilePath() -> String {
    switch self {
      case .americanChemicalSociety:
        return "american-chemical-society"
      case .americanInstituteOfPhysics:
        return "american-institute-of-physics"
      case .americanMedicalAssociation:
        return "american-medical-association"
      case .americanPoliticalScienceAssociation:
        return "american-political-science-association"
      case .americanSociologicalAssociation:
        return "american-sociological-association"
      case .apa:
        return "apa"
      case .associationForComputingMachinery:
        return "association-for-computing-machinery"
      case .cell:
        return "cell"
      case .chicagoAuthorDate:
        return "chicago-author-date"
      case .chicagoNotesBibliography:
        return "chicago-notes-bibliography"
      case .harvardCiteThemRight:
        return "harvard-cite-them-right"
      case .ieee:
        return "ieee"
      case .modernLanguageAssociation:
        return "modern-language-association"
      case .nature:
        return "nature"
      case .science:
        return "science"
      case .springerVancouver:
        return "springer-vancouver"
    }
  }

  public func toFlusterBibliographyCslFile() -> String {
        let fp = self.toEmbeddedFilePath()
        if let fileURL = Bundle.main.url(forResource: "csl/\(fp)", withExtension: "csl", subdirectory: "bibliography_embedded") {
            do {
                let s = try String(contentsOf: fileURL)
                return s
            } catch {
                print("Error: \(error.localizedDescription)")
                fatalError("Failed to get embedded csl file.")
            }
        } else {
            // The file was not found
            print("File not found.")
            fatalError("Failed to get embedded csl file.")
        }
      // This should never be reached. Make sure to throw fatal error if the embedded file can't be found.
      return ""
  }
}


public func getCslLocaleFileContent() -> String {
    if let fileURL = Bundle.main.url(forResource: "csl_locale/en_us", withExtension: "xml", subdirectory: "bibliography_embedded") {
        do {
            let s = try String(contentsOf: fileURL)
            return s
        } catch {
            print("Error: \(error.localizedDescription)")
            fatalError("Failed to get embedded csl locale file.")
        }
    } else {
        fatalError("Failed to get embedded csl locale file.")
    }
  // This should never be reached. Make sure to throw fatal error if the embedded file can't be found.
  return ""
}
