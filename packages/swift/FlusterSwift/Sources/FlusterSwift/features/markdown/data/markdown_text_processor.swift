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
    
    public func parseAsync(ignoreParsing: [Int] = []) {
//        parse_byk
//        ParseMdxByRegexOpts(body: body)
        parseMdxStringByRegex(content: self.body, ignoreParsing: nil)
   
    /// -- Deprecated: Don't use this thing. Use the async version that handles the pre-parsing wiith rust. This method doesn't handle it at all.
    public func parse() {
        for processor in processors {
             processor.parseMarkdown(mdxText: self)
        }
    }
}
