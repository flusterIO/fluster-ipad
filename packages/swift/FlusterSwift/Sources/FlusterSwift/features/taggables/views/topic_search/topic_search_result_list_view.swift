//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/9/25.
//

import SwiftData
import SwiftUI

public struct TopicSearchResultListView: View {
    @Query(sort: \TopicModel.lastAccess, order: .reverse) private var topics:
        [TopicModel]
    @Environment(ThemeManager.self) private var themeManager: ThemeManager
    @Environment(\.modelContext) private var modelContext: ModelContext
    @State private var showDeleteConfirmation: Bool = false
    @State private var topicToDelete: TopicModel?
    @Binding var topicQuery: String
    @Binding var editingNote: NoteModel?
    public init(topicQuery: Binding<String>, editingNote: Binding<NoteModel?>) {
        self._topicQuery = topicQuery
        self._editingNote = editingNote
        if !topicQuery.wrappedValue.isEmpty {
            let t = topicQuery.wrappedValue
            _topics = Query(
                filter: #Predicate<TopicModel> { topic in
                    topic.value.localizedStandardContains(t)
                },
                sort: [
                    SortDescriptor(\TopicModel.lastAccess, order: .reverse)
                ],
                animation: .default
            )
        }
    }
    public var body: some View {
        if topics.isEmpty && topicQuery.isEmpty {
            NoNotesFoundView(
                title: "No topics found",
                subtitle:
                    "Add a topic to your note to filter your notes by topic here."
            )
            .navigationTitle("Topics")
        } else {
            List(topics) { topic in
                NavigationLink(
                    destination: {
                        NoteSearchResultsByTopicView(
                            topic: topic,
                            editingNote: $editingNote
                        )
                    },
                    label: {
                        Text(topic.value)
                    }
                )
                .swipeActions(
                    edge: .leading,
                    content: {
                        Button(
                            action: {
                                topicToDelete = topic
                                showDeleteConfirmation = true
                            },
                            label: {
                                Label("Delete", systemImage: "trash")
                            }
                        )
                        .tint(themeManager.theme.destructive)
                    },
                )
            }
            .searchable(text: $topicQuery, prompt: "Search Topics")
            .navigationTitle("Topics")
            .confirmationDialog(
                "deleteTopicConfirmation",
                isPresented: $showDeleteConfirmation,
                actions: {
                    Button(action: {
                        Task {
                            if let _topicToDelete = topicToDelete {
                                await deleteTopic(_topicToDelete)
                            }
                        }
                    }, label: {
                        Label("Delete", systemImage: "trash")
                    })
                    .tint(themeManager.theme.destructive)
                    .foregroundStyle(themeManager.theme.destructive_foreground)
                },
                message: {
                    Text("Are you sure you want to remove this subject? This will automatically remove this subject from all associated notes.")
                }
            )
        }
    }
    func deleteTopic(_ topic: TopicModel) async {
        let topicValue = topic.value
        let noteFetchDescriptor = FetchDescriptor<NoteModel>(
            predicate: #Predicate<NoteModel> { note in
                note.topic?.value == topicValue
            }
        )
        
        do {
            let noteResults = try modelContext.fetch(noteFetchDescriptor)
            for n in noteResults {
                n.topic = nil
            }
        } catch {
            print("An error occurred while deleting tags \(error)")
        }
        
        modelContext.delete(topic)
    }
}

#Preview {
    TopicSearchResultListView(
        topicQuery: .constant(""),
        editingNote: .constant(nil)
    )
}
