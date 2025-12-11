import FlusterRust
import Foundation
import SwiftData
import FlatBuffers

extension String {
    public func toQuotedJavascriptString() -> String {
        return
            "`\(self.replacingOccurrences(of: "\\", with: "\\\\").replacingOccurrences(of: "`", with: "\\`"))`"
    }
    
    public func isTrimmedEmpty() -> Bool {
        return self.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty
    }

    @MainActor
    public func preParseAsMdxToBytes() async -> Data? {
        let modelContainer = AppDataContainer.shared
        let modelContext = modelContainer.sharedModelContainer.mainContext
        let handler = modelContainer.dataHandlerCreator()
        do {
            return try await parseMdxStringByRegex(
                opts: ParseMdxOptions(content: self, citations: [])
            )
        } catch {
            print("Mdx parsing error: \(error.localizedDescription)")
        }
        return nil
    }

    @MainActor
    /// This will apply the Fluster specific pre-parsers to any string asyncrhonously.
    public func preParseAsMdx() async -> MdxSerialization_MdxParsingResultBuffer? {
        return try? await self.preParseAsMdxToBytes()?.toMdxParsingResult()
    }
}
