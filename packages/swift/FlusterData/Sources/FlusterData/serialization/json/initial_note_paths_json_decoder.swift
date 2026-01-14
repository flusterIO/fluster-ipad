//
//  initial_note_paths_json_decoder.swift
//  Fluster
//
//  Created by Andrew on 11/27/25.
//

import FlusterSwiftMdxParser
import Foundation

public struct InitialNotesDataJsonDecoder {
  public static func decode(from fileName: String) -> [MdxParsingResult]? {
    do {
      if let url = Bundle.main.url(
        forResource: fileName,
        withExtension: "json"
      ) {
        let data = try Data(contentsOf: url)
        let parsingResults = try JSONDecoder().decode(
          [MdxParsingResult].self,
          from: data
        )
        return parsingResults
      } else {
        return nil
      }
    } catch {
      return nil
    }
  }

  public static func loadInitialNoteFromPath(
    notePath: String,
    fileExtension: String = "mdx"
  ) -> String {
    if let filepath = Bundle.main.path(
      forResource: notePath,
      ofType: fileExtension
    ) {
      do {
        let contents = try String(
          contentsOfFile: filepath,
          encoding: .utf8
        )
        return contents
      } catch {
        fatalError(
          "An error occurred while loading initial file contents"
        )
      }
    } else {
      fatalError(
        "An error occurred while loading initial file contents. File was not found."
      )
    }
  }
}
