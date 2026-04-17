import { type Diagnostic, linter } from "@codemirror/lint";
import { tidy } from "bibtex-tidy";

export const bibtexLinter = linter((view): Diagnostic[] => {
    const diagnostics: Diagnostic[] = [];
    const currentDocText = view.state.doc.toString();
    try {
        tidy(currentDocText, { modify: false });
    } catch (_err: unknown) {
        const err = _err as { pos: number }
        // const errNode = view.domAtPos(err.pos);
        diagnostics.push({
            from: err.pos,
            to: view.state.toText(currentDocText).lineAt(err.pos).to,
            severity: "error",
            message: "Invalid BibTeX Syntax",
        });
    }
    return diagnostics;
});
