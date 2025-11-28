//
//  initial_note_paths_json_decoder.swift
//  Fluster
//
//  Created by Andrew on 11/27/25.
//

import Foundation

public struct InitialNoteModelPathJsonDecoder {
    public static func decode(from fileName: String) -> [String] {
        guard
            let url = Bundle.main.url(
                forResource: fileName,
                withExtension: "json"
            ),
            let data = try? Data(contentsOf: url),
            let paths = try? JSONDecoder().decode([String].self, from: data)
        else {
            return []
        }

        return paths
    }
    
    public static func loadInitialNoteFromPath(notePath: String, fileExtension: String = "mdx") -> String {
        if let filepath = Bundle.main.path(forResource: notePath, ofType: fileExtension) {
            do {
                let contents = try String(contentsOfFile: filepath, encoding: .utf8)
                return contents
            } catch {
                fatalError("An error occurred while loading initial file contents")
            }
        } else {
            fatalError("An error occurred while loading initial file contents. File was not found.")
        }
    }
}
