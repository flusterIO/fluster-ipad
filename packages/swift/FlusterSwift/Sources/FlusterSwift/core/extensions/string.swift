import Foundation
import FlusterRust
import SwiftData

public extension String {
    func toQuotedJavascriptString() -> String {
        return "`\(self.replacingOccurrences(of: "\\", with: "\\\\").replacingOccurrences(of: "`", with: "\\`"))`"
    }
   
    /// This will apply the Fluster specific pre-parsers to any string asyncrhonously.
    func preParseAsMdx(modelContext: ModelContext) async -> MdxParsingResult? {
         do {
             let citations = try modelContext.fetch(FetchDescriptor(
                predicate: #Predicate<BibEntryModel> {
                    $0.id != ""
                },
             ))
             let citationSummaries: [SwiftDataCitationSummary] = citations.map({ cit in
                 cit.toCitationSummary()
             })
             return try await parseMdxStringByRegex(opts: ParseMdxOptions(content: self, citations: citationSummaries))
         } catch {
             print("Mdx parsing error: \(error.localizedDescription)")
         }
         return nil
    }
}
