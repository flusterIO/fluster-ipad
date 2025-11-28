//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 11/28/25.
//

import Foundation

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
    
    public func parse() {
        for processor in processors {
             processor.parseMarkdown(mdxText: self)
        }
    }
}
