import Foundation
import FlusterRust
import SwiftData

public extension String {
    func toQuotedJavascriptString() -> String {
        return "`\(self.replacingOccurrences(of: "\\", with: "\\\\").replacingOccurrences(of: "`", with: "\\`"))`"
    }
  
    @MainActor
    /// This will apply the Fluster specific pre-parsers to any string asyncrhonously.
    func preParseAsMdx() async -> MdxParsingResult? {
        let modelContainer = AppDataContainer.shared
        let modelContext = modelContainer.sharedModelContainer.mainContext
        let handler = modelContainer.dataHandlerCreator()
         do {
             // TODO: Fix this and generate MdxParsingResult, serialize it and send it to the webview.
//             let citations = try modelContext.fetch(FetchDescriptor(
//                predicate: #Predicate<BibEntryModel> {
//                    $0.id != ""
//                },
//             ))
//             let citationSummaries: [SwiftDataCitationSummary] = citations.map({ cit in
//                 cit.toCitationSummary()
//             })
             return try await parseMdxStringByRegex(opts: ParseMdxOptions(content: self, citations: []))
         } catch {
             print("Mdx parsing error: \(error.localizedDescription)")
         }
         return nil
    }
}
