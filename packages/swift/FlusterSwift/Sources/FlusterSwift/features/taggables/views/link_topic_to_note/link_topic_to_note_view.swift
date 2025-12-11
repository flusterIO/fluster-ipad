//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/4/25.
//

import SwiftData
import SwiftUI

public struct LinkTopicToNoteView: View {
    @Environment(\.dismiss) private var dismiss
    @Query private var topics: [TopicModel]
    @State private var searchQuery: String = ""
    @State private var showTopicSearch: Bool = false
    @Binding var selection: TopicModel?
    var filteredTopics: [TopicModel] {
        return searchQuery.isEmpty ? topics : topics.filter { $0.value.contains(searchQuery) }
    }
    public init(selection: Binding<TopicModel?>) {
        self._selection = selection
    }
    public var body: some View {
        if filteredTopics.isEmpty {
            NoNotesFoundView(
                title: "No topics found",
                subtitle: searchQuery.isEmpty
                    ? "Tap the + button to create a new topic"
                    : "No topics found that match your query"
            )
            .toolbar {
                ToolbarItem(content: {
                    NavigationLink(
                        destination: {
                            CreateTopicView(selectedTopic: $selection)
                        },
                        label: {
                            Label("Create", systemImage: "plus")
                        }
                    )

                })
            }
        } else {
            HStack(
                alignment: .top,
                content: {
                    List(filteredTopics) { topic in
                        Text(topic.value)
                            .onTapGesture {
                                selection = topic 
                                dismiss()
                            }
                    }
                    .onAppear {
                        showTopicSearch = true
                    }
                    .onDisappear {
                        showTopicSearch = false
                    }
                }
            )
            .toolbar {
                ToolbarItem(content: {
                    NavigationLink(
                        destination: {
                            CreateTopicView(selectedTopic: $selection)
                        },
                        label: {
                            Label("Create", systemImage: "plus")
                        }
                    )

                })
            }
            .pickerStyle(.navigationLink)
            .searchable(
                text: $searchQuery,
                isPresented: $showTopicSearch,
                placement: .automatic,
                prompt: LocalizedStringResource("Search topics")
            )
            .searchable(text: $searchQuery)
        }
    }
}

#Preview {
    LinkTopicToNoteView(selection: .constant(nil))
}
