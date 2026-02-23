//
//  paper_view.swift
//  Fluster
//
//  Created by Andrew on 2/17/26.
//

import FlusterData
import PaperKit
import PencilKit
import SwiftUI

struct PaperView: View {
  @Binding public var editingNote: NoteModel
  @State private var focusedPageIndex: Int = 0
  @State private var showDeletePageConfirmation: Bool = false
  var body: some View {
    GeometryReader { geometry in
      Group {
        if focusedPageIndex < editingNote.paper.count {
          let item = editingNote.paper[focusedPageIndex]
          if let _markup = try? PaperMarkup(dataRepresentation: item.markup) {
            let markup = Binding<PaperMarkup>(
              get: {
                _markup
              },
              set: { newValue in
                Task(priority: .userInitiated) {
                  await handlePaperMarkupChange(newValue)
                }
              })
            PaperMarkupView(markup: markup, focusedIndex: $focusedPageIndex)
              .toolbar(content: {
                ToolbarItem(
                  placement: .primaryAction,
                  content: {
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
                  })
                ToolbarItem(
                  placement: .primaryAction,
                  content: {
                    Button(
                      action: {
                        if focusedPageIndex < editingNote.paper.count - 1 {
                          focusedPageIndex = focusedPageIndex + 1
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
                    .disabled(focusedPageIndex >= editingNote.paper.count - 1)
                    .onAppear {
                      print(focusedPageIndex, editingNote.paper.count - 1)
                    }
                  })
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
                            editingNote.paper.remove(at: focusedPageIndex)
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
                    .disabled(editingNote.paper.count <= 1)
                  })
                ToolbarItem(
                  placement: .primaryAction,
                  content: {
                    Button(
                      action: {
                        focusedPageIndex = editingNote.paper.count
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
              })
          }
        } else {
          Color.clear
        }
      }
      .navigationTitle("Paper \(focusedPageIndex + 1) of \(editingNote.paper.count)")
      .onChange(
        of: focusedPageIndex,
        {
          Task {
            await handleFocusIndexPageCreation(geometry)
          }
        }
      )
      .task {
        await handleFocusIndexPageCreation(geometry)
      }
    }
  }

  func handlePaperMarkupChange(_ markup: PaperMarkup) async {
    if focusedPageIndex < editingNote.paper.count && focusedPageIndex >= 0 {
      do {
        let data = try await markup.dataRepresentation()
        print("Saving editing note markup as data representation...")
        editingNote.paper[focusedPageIndex].markup = data
      } catch {
        print("Error: \(error.localizedDescription)")
      }
    } else {
      print("Attempted to set paper markup on a non-existent sheet.")
    }
  }

  func handleFocusIndexPageCreation(_ geometry: GeometryProxy) async {
    if focusedPageIndex >= editingNote.paper.count {
      let markup = PaperMarkup(bounds: geometry.frame(in: .local))
      do {
        let data = try await markup.dataRepresentation()
        self.editingNote.paper.append(PaperModel(markup: data))
      } catch {
        print("Error handling paper creation: \(error.localizedDescription)")
      }
    }
  }
}

struct PaperMarkupView: NSViewControllerRepresentable {
  typealias NSViewControllerType = MacPaperNsViewController
  @Binding public var markup: PaperMarkup
  @Binding public var focusedIndex: Int

  func makeNSViewController(context: Context) -> MacPaperNsViewController {
    let container = MacPaperNsViewController(markup: $markup, focusedIndex: $focusedIndex)
    return container
  }

  func updateNSViewController(_ nsViewController: MacPaperNsViewController, context: Context) {
    context.coordinator.parent = self
//    markup = nsViewController.markup
    //      context.coordinator.
    //    nsViewController.markup = markup
    //      if let c = self.controller {
    //          c.markup = markup
    //      }
    //    if focusedIndex != previousFocusedIndex {
    //      previousFocusedIndex = focusedIndex
    //      nsViewController.markup = markup
    //    }
  }

  private func resetMarkupContent(on nsViewController: MacPaperNsViewController) {
    print("Resetting PaperMarkup content for page index: \(focusedIndex)")
    nsViewController.update(with: markup)
  }

  func makeCoordinator() -> Coordinator {
    Coordinator(self)
  }

  class Coordinator: NSObject {
    var parent: PaperMarkupView
    init(_ parent: PaperMarkupView) { self.parent = parent }
  }
}
