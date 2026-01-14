import FlatBuffers
import FlusterSwiftMdxParser
import Foundation
import SwiftData
import FlusterData

extension String {
  public func toQuotedJavascriptString() -> String {
    return
      "`\(self.replacingOccurrences(of: "\\", with: "\\\\").replacingOccurrences(of: "`", with: "\\`"))`"
  }

  public func isTrimmedEmpty() -> Bool {
    return self.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty
  }

  @MainActor
    public func preParseAsMdxToBytes(noteId: String?) async -> Data? {
    let modelContainer = AppDataContainer.shared
    let modelContext = modelContainer.sharedModelContainer.mainContext
    let handler = modelContainer.dataHandlerCreator()
    do {
      return try await parseMdxStringByRegex(
        opts: ParseMdxOptions( noteId: noteId, content: self, citations: [])
      )
    } catch {
      print("Mdx parsing error: \(error.localizedDescription)")
    }
    return nil
  }

  @MainActor
  /// This will apply the Fluster specific pre-parsers to any string asyncrhonously.
    public func preParseAsMdx(noteId: String?) async -> MdxSerialization_MdxParsingResultBuffer? {
        return try? await self.preParseAsMdxToBytes(noteId: noteId)?.toMdxParsingResult()
  }
}
