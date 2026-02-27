import FlatBuffers
import FlusterData
import FlusterSwiftMdxParser
import Foundation
import SwiftData

extension String {
  public func toQuotedJavascriptString() -> String {
    return
      "`\(self.replacingOccurrences(of: "\\", with: "\\\\").replacingOccurrences(of: "`", with: "\\`"))`"
  }

  public func isTrimmedEmpty() -> Bool {
    return self.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty
  }

  public func toFlatBufferSerializedString() -> [UInt8] {
    var builder = FlatBufferBuilder(initialSize: 1024)

    let bodyOffset = builder.create(string: self)
    let serializedStringStart = SharedWebviewData_SerializedString.startSerializedString(&builder)
    SharedWebviewData_SerializedString.add(body: bodyOffset, &builder)
    let serializedString = SharedWebviewData_SerializedString.endSerializedString(
      &builder, start: serializedStringStart)
    builder.finish(offset: serializedString)
    return builder.sizedByteArray
  }

  @MainActor
  /// This will apply the Fluster specific pre-parsers to any string asyncrhonously.
  public func preParseAsMdx(noteId: String?) async -> MdxParsingResult? {
    do {
      let res = await FlusterSwiftMdxParser.preParseMdx(
        options: ParseMdxOptions(noteId: noteId, content: self, citations: []))
      return res
    } catch {
      print("Error: \(error.localizedDescription)")
    }
    return nil
  }
}
