//
//  mdx_text_utils.swift
//  Fluster
//
//  Created by Andrew on 11/27/25.
//

import Foundation

public struct MdxTextUtils {
    public static func removeHeadingSyntax(val: String) -> String {
        if val.hasPrefix("#") {
            return removeHeadingSyntax(val: String(val.suffix(val.count - 1)))
        }
        return val.trimmingCharacters(in: NSCharacterSet.whitespacesAndNewlines)
    }
    public static func parseMarkdownTitle(titleString: String, depth: Int = 0)
        -> MdxTitleResult
    {
        if !titleString.hasPrefix("#") {
            return MdxTitleResult(depth: depth, content: titleString)
        }
        let result = parseMarkdownTitle(
            titleString: String(titleString.dropFirst()),
            depth: depth + 1
        )
        if result.depth > 6 {
            return MdxTitleResult(depth: 0, content: titleString)
        } else {
            return result
        }
    }
    
    public static func getTitles(body: String) -> [MdxTitleResult] {
        let lines = body.components(separatedBy: .newlines)
        var titles: [MdxTitleResult] = []
        lines.forEach { line in
            if line.hasPrefix("#") {
                let res = parseMarkdownTitle(titleString: line)
                titles.append(res)
            }
        }
        return titles
    }

    public static func getTitle(body: String) -> String? {
        let lines = body.components(separatedBy: .newlines)
        let firstTitleLine = lines.first(where: { $0.hasPrefix("#") })
        if firstTitleLine == nil {
            return nil
        }
        return removeHeadingSyntax(val: firstTitleLine!)
    }

    public static func getHighestMatch(titleResults: [MdxTitleResult], query: String)
        -> MdxTitleResult?
    {
        var matches: [MdxTitleResult] = []
        for match in titleResults {
            if match.content.localizedCaseInsensitiveContains(query) {
                matches.append(match)
            }
        }
        return matches.sorted { $0.depth < $1.depth }.first
    }

    /// This function assumes that all 'notes' have been validated to be a match by the database query. Otherwise  all notes will return to be a match.
    public static func sortNotesByMarkdownBodyMatch(notes: [NoteModel], query: String)
        -> [NoteModel]
    {
        var h1Results: [NoteModel] = []
        var h2Results: [NoteModel] = []
        var h3Results: [NoteModel] = []
        var h4Results: [NoteModel] = []
        var h5Results: [NoteModel] = []
        var h6Results: [NoteModel] = []
        var bodyResults: [NoteModel] = []
        for note in notes {
            let titles = getTitles(body: note.markdown.body)
            print("Titles \(titles)")
            if !titles.isEmpty {
                if let highestMatch = getHighestMatch(
                    titleResults: titles,
                    query: query
                ) {
                    switch highestMatch.depth {
                    case 0:
                        bodyResults.append(note)
                    case 1:
                        h1Results.append(note)
                    case 2:
                        h2Results.append(note)
                    case 3:
                        h3Results.append(note)
                    case 4:
                        h4Results.append(note)
                    case 5:
                        h5Results.append(note)
                    case 6:
                        h6Results.append(note)
                    default:
                        bodyResults.append(note)
                    }
                } else {
                    bodyResults.append(note)
                }
            }
        }
        
        print("H1 match length \(h1Results.count)")
        
        return h1Results + h2Results + h3Results + h4Results + h5Results
            + h6Results + bodyResults
    }

}
