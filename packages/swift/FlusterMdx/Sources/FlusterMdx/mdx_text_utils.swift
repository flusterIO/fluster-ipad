import Foundation

public struct MdxTextUtils {
  public static func removeHeadingSyntax(val: String) -> String {
    if val.hasPrefix("#") {
      return removeHeadingSyntax(val: String(val.suffix(val.count - 1)))
    }
    return val.trimmingCharacters(in: NSCharacterSet.whitespacesAndNewlines)
  }
    public static func removeSurroundingQuotes(val: String) -> String {
        if val.hasPrefix("\"") && val.hasSuffix("\"") {
            return String(val.dropFirst().dropLast())
        }
        if val.hasPrefix("'") && val.hasSuffix("'") {
            return String(val.dropFirst().dropLast())
        }
        return val
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
      return removeSurroundingQuotes(val: removeHeadingSyntax(val: firstTitleLine!))
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
}
