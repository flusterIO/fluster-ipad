// public struct MarkdownHeadingStringifiedResult {
//     public var depth: UInt16
//     public var content: String
//     public var id: String?

//     // Default memberwise initializers are never public by default, so we
//     // declare one manually.
//     public init(depth: UInt16, content: String, id: String?) {
//         self.depth = depth
//         self.content = content
//         self.id = id
//     }
// }

import ConundrumSwift

extension MarkdownHeadingStringifiedResult: @retroactive Codable {
  public enum CodingKeys: String, CodingKey {
    case depth, content, id, tabDepth
  }

  public init(from decoder: Decoder) throws {
    let container = try decoder.container(keyedBy: CodingKeys.self)
    let depth = try container.decode(UInt16.self, forKey: .depth)
    let tabDepth = try container.decode(UInt16.self, forKey: .tabDepth)
    let content = try container.decode(String.self, forKey: .content)
    let id = try container.decode(String.self, forKey: .id)
    self.init(
      depth: depth,
      tabDepth: tabDepth,
      content: content,
      id: id
    )
  }

  public func encode(to encoder: any Encoder) throws {
    var container = encoder.container(keyedBy: CodingKeys.self)

    try container.encode(depth, forKey: .depth)
    try container.encode(tabDepth, forKey: .tabDepth)
    try container.encode(content, forKey: .content)
    try container.encode(id, forKey: .id)
  }
}
