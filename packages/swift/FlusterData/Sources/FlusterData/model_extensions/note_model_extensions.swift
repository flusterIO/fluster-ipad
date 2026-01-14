import FlusterMdx
import Foundation

extension Array where Element: NoteModel {
  public func sortNotesByMarkdownBodyMatch(
    query: String, filterNoMatch: Bool = true
  )
    -> [NoteModel]
  {
    var h1Results: [NoteModel] = []
    var h2Results: [NoteModel] = []
    var h3Results: [NoteModel] = []
    var h4Results: [NoteModel] = []
    var h5Results: [NoteModel] = []
    var h6Results: [NoteModel] = []
    var bodyResults: [NoteModel] = []
    self.forEach { note in
      let titles = MdxTextUtils.getTitles(body: note.markdown.body)
      if !titles.isEmpty {
          if let highestMatch = MdxTextUtils.getHighestMatch(
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
        } else if !filterNoMatch {
          bodyResults.append(note)
        }
      }
    }

    return h1Results + h2Results + h3Results + h4Results + h5Results
      + h6Results + bodyResults
  }
}
