//
//  paper_tab_view.swift
//  Fluster
//
//  Created by Andrew on 2/22/26.
//

import FlusterData
import FlusterSwift
import PencilKit
import SwiftUI

struct PaperTabView: View {
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
                })
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
                })
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
                        editingNote?.paper.remove(at: focusedPageIndex)
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
