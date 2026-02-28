import FlatBuffers
import FlusterData
import FlusterSwiftMdxParser
import Foundation
import SwiftData

extension String {
  /// Returns a string that's able to be sent through a javascript string that's *already* surrounded by quotes.
  public func toQuotedJavascriptString() -> String {
    func backupParseString(s: String) -> String {
      return
        "`\(s.replacingOccurrences(of: "\\", with: "\\\\").replacingOccurrences(of: "`", with: "\\`"))`"
    }
    do {
      // Encode the string directly into JSON data
      let data = try JSONEncoder().encode(self)

      // Convert the data back to a Swift String
      guard let jsonString = String(data: data, encoding: .utf8) else {
        return backupParseString(s: self)
      }

      // Note: jsonString already includes the bounding double quotes ("")
      return jsonString
    } catch {
      print("Failed to escape string: \(error)")
      return backupParseString(s: self)
    }
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
      let res = try await preParseMdx(
        options: ParseMdxOptions(noteId: noteId, content: self, citations: []))
      return res
    } catch {
      print("Error: \(error.localizedDescription)")
    }
    return nil
  }
}
