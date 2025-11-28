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
        
        mdxText.body.matches(of: regex).forEach { match in
            print("Match: \(match)")
        }
        
    }
}
