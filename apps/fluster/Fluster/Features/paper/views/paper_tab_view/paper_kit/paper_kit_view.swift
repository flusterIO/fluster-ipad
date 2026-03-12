//
//  paper_kit_view.swift
//  Fluster
//
//  Created by Andrew on 2/22/26.
//

import FlusterData
import FlusterSwift
import PaperKit
import SwiftUI

struct PaperKitView: View {
  @Binding public var editingNote: NoteModel
  @Binding public var focusedPageIndex: Int
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
            PaperCanvasView(
              markup: markup,
              canvasBounds: geometry.frame(in: .local),
              onContentChanged: { newContent in
                print("Changed?")
              }
            )
          } else {
            Color.clear
          }
        } else {
          Color.clear.task {
            await handleFocusIndexPageCreation(geometry)
          }
        }
      }
      .navigationTitle(
        "Paper \(_item?.pageIndex == nil ? 1 : _item!.pageIndex + 1) of \(maxPageIndex + 1)"
      )
      .navigationSubtitle("\(editingNote.paper.count) pages total")
    }
  }
  func handleFocusIndexPageCreation(_ geometry: GeometryProxy) async {
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
  func handlePaperMarkupChange(_ markup: PaperMarkup) async {
    if focusedPageIndex < editingNote.paper.count && focusedPageIndex >= 0 {
      do {
        editingNote.setLastRead(setModified: true)
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
}
