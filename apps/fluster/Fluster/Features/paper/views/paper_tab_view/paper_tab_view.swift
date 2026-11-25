//
//  paper_tab_view.swift
//  Fluster
//
//  Created by Andrew on 2/22/26.
//

import FlusterData
import FlusterSwift
import PencilKit
import SwiftData
import SwiftUI

struct PaperTabView: View {
  @Environment(\.modelContext) private var modelContext: ModelContext
  @State private var toolbar: PKToolPicker = PKToolPicker()
  @State private var showDeletePageConfirmation: Bool = false
  @Environment(ThemeManager.self) private var themeManager: ThemeManager
  @AppStorage(AppStorageKeys.hasLaunchedPreviously.rawValue) private
    var hasPreviouslyLaunched: Bool = false
  @Binding var editingNote: NoteModel?
  @State private var focusedPageIndex: Int = 0
  init(
    editingNote: Binding<NoteModel?>,
  ) {
    self._editingNote = editingNote
  }
  var body: some View {
    NavigationStack {
      if let en = Binding($editingNote) {
        PaperKitView(editingNote: en, focusedPageIndex: $focusedPageIndex)
          .id("\(en.wrappedValue.id)-\(focusedPageIndex)")
          .onChange(of: en.wrappedValue.id) { _, _ in
            focusedPageIndex = 0
          }
          .toolbar {
            ToolbarItem(id: "prev-page", placement: .primaryAction) {
              Button(
                action: {
                  if focusedPageIndex >= 1 {
                    focusedPageIndex = focusedPageIndex - 1
                  }
                },
                label: {
                  Label(
                    title: {
                      Text("Previous")
                    },
                    icon: {
                      Image(systemName: "chevron.left")
                    })
                }
              )
              .disabled(focusedPageIndex <= 0)
            }
            ToolbarItem(id: "next-page", placement: .primaryAction) {
              Button(
                action: {
                  if let en = editingNote {
                    if focusedPageIndex < en.paper.count - 1 {
                      focusedPageIndex = focusedPageIndex + 1
                    }
                  }
                },
                label: {
                  Label(
                    title: {
                      Text("Next")
                    },
                    icon: {
                      Image(systemName: "chevron.right")
                    })
                }
              )
              .disabled(focusedPageIndex >= en.paper.count - 1)
            }
            ToolbarItem(
              placement: .destructiveAction,
              content: {
                Button(
                  action: {
                    showDeletePageConfirmation = true
                  },
                  label: {
                    Label(
                      title: {
                        Text("Delete Page")
                      },
                      icon: {
                        Image(systemName: "trash")
                      })
                  }
                )
                .confirmationDialog(
                  "Are you sure?", isPresented: $showDeletePageConfirmation,
                  actions: {
                    Button(
                      action: {
                        // focusedPageIndex indexes the pages sorted by pageIndex, which is not
                        // necessarily the storage order of `paper`, so resolve the page first.
                        if let note = editingNote {
                          let sorted = note.paper.sorted(by: { $0.pageIndex < $1.pageIndex })
                          if focusedPageIndex < sorted.count,
                            let storageIndex = note.paper.firstIndex(where: {
                              $0 === sorted[focusedPageIndex]
                            })
                          {
                            let page = note.paper.remove(at: storageIndex)
                            // Removing from the array only unlinks the page; delete the model so
                            // it isn't orphaned in the store.
                            modelContext.delete(page)
                          }
                        }
                        if focusedPageIndex > 0 {
                          focusedPageIndex = focusedPageIndex - 1
                        } else {
                          focusedPageIndex = 0
                        }
                      },
                      label: {
                        Label(
                          title: {
                            Text("Delete")
                          },
                          icon: {
                            Image(systemName: "trash")
                          })
                      }
                    )
                  }
                )
                .disabled(en.paper.count <= 1)
              })
            ToolbarItem(
              placement: .primaryAction,
              content: {
                Button(
                  action: {
                    focusedPageIndex = en.paper.count
                  },
                  label: {
                    Label(
                      title: {
                        Text("Create")
                      },
                      icon: {
                        Image(systemName: "plus")
                      })
                  })
              })
          }
      } else {
        NoNotesFoundView()
      }
    }
  }
}
