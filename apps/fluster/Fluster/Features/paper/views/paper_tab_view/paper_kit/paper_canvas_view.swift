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
    // 1. Initialize the Markup model
    let markup: PaperMarkup
    if !canvasData.isEmpty, let restored = try? PaperMarkup(dataRepresentation: canvasData) {
      markup = restored
    } else {
      markup = PaperMarkup(bounds: canvasBounds)
    }

    let features: FeatureSet = .latest

    let paperVC = PaperMarkupViewController(markup: markup, supportedFeatureSet: features)
    //    paperVC.delegate = PaperCanvasViewController()
    let container = PaperContainerViewController(paperVC: paperVC)

    return container
  }

  func updateUIViewController(_ uiViewController: PaperContainerViewController, context: Context) {
    // Handle external updates to canvasData if necessary
    print("Update in uiviewcontroller?")
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
      Task {
        if let newData = try await controller.markup?.dataRepresentation() {
          parent.canvasData = newData
          parent.onContentChanged?(newData)
        }
      }
    }
  }
}

// MARK: - Specialized Container
class PaperContainerViewController: UIViewController {
  let paperVC: PaperMarkupViewController
  private let toolPicker = PKToolPicker()

  init(paperVC: PaperMarkupViewController) {
    self.paperVC = paperVC
    super.init(nibName: nil, bundle: nil)
  }

  required init?(coder: NSCoder) { fatalError("init(coder:) has not been implemented") }

  override func viewDidLoad() {
    super.viewDidLoad()
    view.backgroundColor = .systemBackground

    addChild(paperVC)
    view.addSubview(paperVC.view)
    paperVC.didMove(toParent: self)

    NSLayoutConstraint.activate([
      paperVC.view.topAnchor.constraint(equalTo: view.topAnchor),
      paperVC.view.bottomAnchor.constraint(equalTo: view.bottomAnchor),
      paperVC.view.leftAnchor.constraint(equalTo: view.leftAnchor),
      paperVC.view.rightAnchor.constraint(equalTo: view.rightAnchor),
    ])
    // 4. Critical: Link ToolPicker to the markup controller
    toolPicker.addObserver(paperVC)
    toolPicker.setVisible(true, forFirstResponder: paperVC)
  }

  override func viewDidLayoutSubviews() {
    super.viewDidLayoutSubviews()
    paperVC.view.frame = view.bounds
  }

  override func viewDidAppear(_ animated: Bool) {
    super.viewDidAppear(animated)
    // 5. Must be first responder to activate the drawing engine
    paperVC.becomeFirstResponder()
  }
}

// MARK: - Notification Extension
// Ensures we are using the correct string-based notification for 2026 PaperKit
extension Notification.Name {
  static let PaperMarkupViewControllerDidChange = Notification.Name(
    "PaperMarkupViewControllerDidChangeNotification")
}
