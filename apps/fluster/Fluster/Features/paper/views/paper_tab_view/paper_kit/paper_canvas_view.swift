import SwiftUI
import PaperKit
import PencilKit

struct PaperCanvasView: UIViewControllerRepresentable {
    @Binding var canvasData: Data
    
    func makeUIViewController(context: Context) -> PaperContainerViewController {
        // 1. Initialize with the screen bounds to ensure non-zero size
        let markup = PaperMarkup(bounds: UIScreen.main.bounds)
        let paperVC = PaperMarkupViewController(markup: markup, supportedFeatureSet: .latest)
        
        // 2. Use our custom container to manage the lifecycle
        let container = PaperContainerViewController(paperVC: paperVC)
        context.coordinator.markupController = paperVC
        
        return container
    }
    
    func updateUIViewController(_ uiViewController: PaperContainerViewController, context: Context) {
        // Update frames if needed
    }
    
    func makeCoordinator() -> Coordinator {
        Coordinator(self)
    }
    
    class Coordinator: NSObject {
        var parent: PaperCanvasView
        weak var markupController: PaperMarkupViewController?
        init(_ parent: PaperCanvasView) { self.parent = parent }
    }
}

// MARK: - Custom Container to Force Lifecycle
class PaperContainerViewController: UIViewController {
    let paperVC: PaperMarkupViewController
    let toolPicker = PKToolPicker()

    init(paperVC: PaperMarkupViewController) {
        self.paperVC = paperVC
        super.init(nibName: nil, bundle: nil)
    }

    required init?(coder: NSCoder) { fatalError("init(coder:) has not been implemented") }

    override func viewDidLoad() {
        super.viewDidLoad()
        view.backgroundColor = .systemBackground
        
        // Add as child
        addChild(paperVC)
        view.addSubview(paperVC.view)
        paperVC.didMove(toParent: self)
        
        // Force the ToolPicker to recognize this specific VC
        toolPicker.addObserver(paperVC)
        toolPicker.setVisible(true, forFirstResponder: paperVC)
    }

    override func viewDidLayoutSubviews() {
        super.viewDidLayoutSubviews()
        // Ensure the paper view matches our container exactly
        paperVC.view.frame = view.bounds
    }

    override func viewDidAppear(_ animated: Bool) {
        super.viewDidAppear(animated)
        // 3. THIS IS THE KEY: PaperKit needs to be First Responder 
        // after the view is officially in the hierarchy to stop being black.
        paperVC.becomeFirstResponder()
    }
}
