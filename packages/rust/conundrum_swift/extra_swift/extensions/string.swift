import Foundation

extension String {
  public func parseConundrumToString(
    modifiers: [ConundrumModifier], uiParams: UiParams, target: ConundrumCompileTarget,
    trusted: Bool
  )
    async throws -> String?
  {
    let res = try await runConundrum(
      options: ParseConundrumOptions(
        noteId: nil, content: self, modifiers: modifiers, hideComponents: [], uiParams: uiParams,
        target: target, trusted: trusted))
    return res.content
  }
}
