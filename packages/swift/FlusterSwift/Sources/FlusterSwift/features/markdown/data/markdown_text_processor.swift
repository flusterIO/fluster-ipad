//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 11/28/25.
//

import Foundation
import FlusterRust

protocol MdxTextPreProcessor {
    func parseMarkdown(mdxText: MdxText)
}

public class MdxText {
    var body: String
    let processors: [MdxTextPreProcessor] = [
        MdxTagParser()
    ]

    public init(body: String) {
        self.body = body
    }

    public func parseAsync(opts: ParseMdxOptions) async -> MdxParsingResult? {
         do {
             return try await parseMdxStringByRegex(opts: opts)
         } catch {
             print("Mdx parsing error: \(error.localizedDescription)")
         }
         return nil
    }

    /// -- Deprecated: Don't use this thing. Use the async version that handles the pre-parsing wiith rust. This method doesn't handle it at all.
    public func parse() {
        print("REMOVE ALL REFERENCES TO THIS METHOD")
        for processor in processors {
            processor.parseMarkdown(mdxText: self)
        }
    }
}
