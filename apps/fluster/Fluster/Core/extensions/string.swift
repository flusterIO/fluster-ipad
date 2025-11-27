import Foundation

extension String {
    func toQuotedJavascriptString() -> String {
        return "`\(self.replacingOccurrences(of: "\\", with: "\\\\").replacingOccurrences(of: "`", with: "\\`"))`"
    }
}
