//
//  mdx_tag_parser.swift
//  FlusterSwift
//
//  Created by Andrew on 11/28/25.
//

import Foundation

class MdxTagParser: MdxTextPreProcessor {

    init() {
    }

    func parseMarkdown(mdxText: MdxText) {
        let regex = /\[\[#(?<body>[^\]]*)]]/
        // let emailRegex = Regex {
        //     OneOrMore(.any) // Username part
        //     "@"
        //     OneOrMore(.any) // Domain part
        //     "."
        //     "com" // Top-level domain
        // }

        mdxText.body.matches(of: regex).forEach { match in
            print("Match: \(match)")
        }

    }
}
