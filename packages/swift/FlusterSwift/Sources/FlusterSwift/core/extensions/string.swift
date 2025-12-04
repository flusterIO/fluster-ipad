import Foundation
import FlusterRust

public extension String {
    func toQuotedJavascriptString() -> String {
        return "`\(self.replacingOccurrences(of: "\\", with: "\\\\").replacingOccurrences(of: "`", with: "\\`"))`"
    }
   
    /// This will apply the Fluster specific pre-parsers to any string asyncrhonously.
    func preParseAsMdx() async -> MdxParsingResult? {
         do {
             return try await parseMdxStringByRegex(opts: ParseMdxOptions(content: self))
         } catch {
             print("Mdx parsing error: \(error.localizedDescription)")
         }
         return nil
    }
}
