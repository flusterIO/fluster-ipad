//
//  paper_view.swift
//  Fluster
//
//  Created by Andrew on 2/17/26.
//

import FlusterData
import PaperKit
import PencilKit
import SwiftData
import SwiftUI

struct PaperView: View {
  @Binding public var editingNote: NoteModel
  @Binding public var focusedPageIndex: Int
  @Environment(\.modelContext) private var modelContext: ModelContext
  @State private var showDeletePageConfirmation: Bool = false
  /// A hack required to avoid updating the utime of the note on initial render.
  @State private var trackChanges: Bool = false
  var sortedPages: [PaperModel] {
    return editingNote.paper.sorted(by: { a, b in
      a.pageIndex < b.pageIndex
    })
  }
  var maxPageIndex: Int {
    if let paper = sortedPages.last {
      paper.pageIndex
    } else {
      -1
    }
  }
  var _item: PaperModel? {
    if focusedPageIndex < sortedPages.count {
      sortedPages[focusedPageIndex]
    } else {
      nil
    }
  }
  var body: some View {
    GeometryReader { geometry in
      Group {
        if let item = _item {
          if let _markup = try? PaperMarkup(dataRepresentation: item.markup) {
            let markup = Binding<PaperMarkup>(
              get: {
                _markup
              },
              set: { newValue in
                Task(priority: .userInitiated) {
                  await handlePaperMarkupChange(newValue)
                }
                return
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
                            let focusedPageIndexPageIndex = sortedPages[focusedPageIndex].pageIndex
                            editingNote.removePaperByPageIndex(
                              pageIndex: focusedPageIndexPageIndex, modelContext: modelContext)
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
      .navigationTitle(
        "Paper \(_item?.pageIndex == nil ? 1 : _item!.pageIndex + 1) of \(maxPageIndex + 1)"
      )
      .navigationSubtitle("\(editingNote.paper.count) pages total")
      .task {
        await handleFocusIndexPageCreation(geometry)
      }
    }
  }

  func handlePaperMarkupChange(_ markup: PaperMarkup) async {
    if focusedPageIndex < editingNote.paper.count && focusedPageIndex >= 0 {
      do {
        if trackChanges {
          editingNote.setLastRead(setModified: true)
        } else {
          trackChanges = true
        }
        let data = try await markup.dataRepresentation()
        print("Saving editing note markup as data representation...")
        sortedPages[focusedPageIndex].markup = data
      } catch {
        print("Error: \(error.localizedDescription)")
      }
    } else {
      print("Attempted to set paper markup on a non-existent sheet.")
    }
  }

  func handleFocusIndexPageCreation(_ geometry: GeometryProxy) async {
    print("Ran here...")
    if focusedPageIndex >= editingNote.paper.count {
      let markup = PaperMarkup(bounds: CGRect(origin: .zero, size: getPaperMarkupBounds()))
      do {
        let data = try await markup.dataRepresentation()
        self.editingNote.paper.append(PaperModel(markup: data, pageIndex: maxPageIndex + 1))
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
    if context.coordinator.lastPageIndex != focusedIndex {
      print("Updating PaperMarkup content for page index: \(focusedIndex)")
      // Clear the canvas before loading new markup content to avoid residual drawings.
      clearAndLoadMarkup(on: nsViewController, with: $markup.wrappedValue)
      context.coordinator.lastPageIndex = focusedIndex
    }
  }

  /// Clears the canvas and loads the specified PaperMarkup content.
  /// - Parameters:
  ///   - nsViewController: The MacPaperNsViewController instance managing the canvas.
  ///   - markup: The PaperMarkup data to load into the canvas.
  private func clearAndLoadMarkup(
    on nsViewController: MacPaperNsViewController, with markup: PaperMarkup
  ) {
    print("Loading markup for page index: \(focusedIndex)")
    // Just update (replace) the visible canvas with the model's markup—do not clear the saved data!
    nsViewController.update(with: markup)
  }

  func makeCoordinator() -> Coordinator {
    Coordinator(self)
  }

  class Coordinator: NSObject {
    var parent: PaperMarkupView
    var lastPageIndex: Int
    init(_ parent: PaperMarkupView) {
      self.parent = parent
      self.lastPageIndex = parent.focusedIndex
    }
  }
}
