import Foundation

extension String {
  public func parseConundrumToString(modifiers: [ConundrumModifier]) async throws -> String? {
      let res = try await runConundrum(
        options: ParseConundrumOptions(noteId: nil, content: self, modifiers: modifiers, hideComponents: []))
      return res.content
  }
}
