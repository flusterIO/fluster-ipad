//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/18/25.
//

import Foundation
import FlusterData

public enum MainFullScreenCover {
  case tagSearch(tag: TagModel)
  case topicSearch(topic: TopicModel)
  case subjectSearch(subject: SubjectModel)
  case citationByKey(citation: BibEntryModel)
}
