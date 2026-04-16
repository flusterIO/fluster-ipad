import Foundation

extension String {
  public func parseConundrumToString(modifiers: [ConundrumModifier], uiParams: UiParams)
    async throws -> String?
  {
    let res = try await runConundrum(
      options: ParseConundrumOptions(
        noteId: nil, content: self, modifiers: modifiers, hideComponents: [], uiParams: uiParams))
    return res.content
  }
}
