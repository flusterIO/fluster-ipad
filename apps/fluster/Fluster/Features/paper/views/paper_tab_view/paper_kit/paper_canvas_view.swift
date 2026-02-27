import PaperKit
import PencilKit
import SwiftUI

struct PaperCanvasView: UIViewControllerRepresentable {
  @Binding var canvasData: Data
  let canvasBounds: CGRect
  var onContentChanged: ((Data) -> Void)?

  func makeCoordinator() -> Coordinator {
    Coordinator(self)
  }

  func makeUIViewController(context: Context) -> PaperContainerViewController {
    let markup: PaperMarkup
    if !canvasData.isEmpty, let restored = try? PaperMarkup(dataRepresentation: canvasData) {
      markup = restored
    } else {
      markup = PaperMarkup(bounds: CGRect(origin: .zero, size: getPaperMarkupBounds()))
    }

    let features: FeatureSet = .latest

    let paperVC = PaperMarkupViewController(markup: markup, supportedFeatureSet: features)
    let container = PaperContainerViewController(paperVC: paperVC, parent: self)

    return container
  }

  func updateUIViewController(_ uiViewController: PaperContainerViewController, context: Context) {
    print("Changed in updateUIViewController")
    // Handle external updates to canvasData if necessary
  }

  // MARK: - Coordinator
  class Coordinator: NSObject {
    var parent: PaperCanvasView
    private var observation: Any?

    init(_ parent: PaperCanvasView) {
      self.parent = parent
    }

    private func handleUpdate(from controller: PaperMarkupViewController) {
      print("Update in handleUpdate")
    }
  }
}

// MARK: - Specialized Container
class PaperContainerViewController: UIViewController, PaperMarkupViewController.Delegate {
  let paperVC: PaperMarkupViewController
  let parentContainer: PaperCanvasView
  private let toolPicker = PKToolPicker()

  init(paperVC: PaperMarkupViewController, parent: PaperCanvasView) {
    self.paperVC = paperVC
    self.parentContainer = parent
    super.init(nibName: nil, bundle: nil)
  }

  required init?(coder: NSCoder) { fatalError("init(coder:) has not been implemented") }

  override func viewDidLoad() {
    super.viewDidLoad()
    view.backgroundColor = .systemBackground

    addChild(paperVC)
    view.addSubview(paperVC.view)
    paperVC.didMove(toParent: self)

    paperVC.delegate = self

    // 4. FIX: You must set this to false, or your NSLayoutConstraints will fail/crash!
    paperVC.view.translatesAutoresizingMaskIntoConstraints = false
    NSLayoutConstraint.activate([
      paperVC.view.topAnchor.constraint(equalTo: view.topAnchor),
      paperVC.view.bottomAnchor.constraint(equalTo: view.bottomAnchor),
      paperVC.view.leftAnchor.constraint(equalTo: view.leftAnchor),
      paperVC.view.rightAnchor.constraint(equalTo: view.rightAnchor)
    ])

    // 5. Critical: Link ToolPicker to the markup controller
    toolPicker.addObserver(paperVC)
    toolPicker.setVisible(true, forFirstResponder: paperVC)
  }

  override func viewDidAppear(_ animated: Bool) {
    super.viewDidAppear(animated)
    paperVC.delegate = self
    // Must be first responder to activate the drawing engine
    paperVC.becomeFirstResponder()
  }

  func paperMarkupViewControllerDidChangeMarkup(
    _ paperMarkupViewController: PaperMarkupViewController
  ) {
    if let _markup = paperMarkupViewController.markup {
      Task {
        if let dataRep = try? await _markup.dataRepresentation() {
          parentContainer.canvasData = dataRep
        }
      }
    }
  }

  func paperMarkupViewControllerDidChangeSelection(
    _ paperMarkupViewController: PaperMarkupViewController
  ) {
  }

  func paperMarkupViewControllerDidBeginDrawing(
    _ paperMarkupViewController: PaperMarkupViewController
  ) {
  }

  func paperMarkupViewControllerDidChangeContentVisibleFrame(
    _ paperMarkupViewController: PaperMarkupViewController
  ) {
  }
}

// MARK: - Notification Extension
extension Notification.Name {
  static let PaperMarkupViewControllerDidChange = Notification.Name(
    "PaperMarkupViewControllerDidChangeNotification")
}
