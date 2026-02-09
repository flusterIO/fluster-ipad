import { Completion, snippetCompletion } from "@codemirror/autocomplete";

export const getMathSnippets = (): Completion[] => {
    const greekLower = [
        "alpha", "beta", "gamma", "delta", "epsilon", "zeta", "eta", "theta",
        "iota", "kappa", "lambda", "mu", "nu", "xi", "omicron", "pi", "rho",
        "sigma", "tau", "upsilon", "phi", "chi", "psi", "omega"
    ];

    const greekUpper = [
        "Gamma", "Delta", "Theta", "Lambda", "Xi", "Pi", "Sigma", "Upsilon",
        "Phi", "Psi", "Omega"
    ];

    const greekVars = [
        "varepsilon", "varphi", "varrho", "vartheta", "varsigma"
    ];

    // Helper to create simple command snippets (e.g., \alpha)
    const createSymbol = (name: string, detail: string = "Greek Letter") => ({
        label: `\\${name}`,
        detail,
        type: "constant",
        apply: `\\${name}`
    });

    // 1. Greek Letters
    const greekSnippets = [
        ...greekLower.map(g => createSymbol(g, "Greek Lower")),
        ...greekUpper.map(g => createSymbol(g, "Greek Upper")),
        ...greekVars.map(g => createSymbol(g, "Greek Variation"))
    ];

    // 2. Common Math Fonts
    const fontSnippets = [
        snippetCompletion("\\mathbb{#{}}", { label: "\\mathbb", detail: "Blackboard Bold (R, Z, C)", type: "function" }),
        snippetCompletion("\\mathcal{#{}}", { label: "\\mathcal", detail: "Calligraphic", type: "function" }),
        snippetCompletion("\\mathfrak{#{}}", { label: "\\mathfrak", detail: "Fraktur", type: "function" }),
        snippetCompletion("\\mathbf{#{}}", { label: "\\mathbf", detail: "Bold Face", type: "function" }),
        snippetCompletion("\\mathrm{#{}}", { label: "\\mathrm", detail: "Roman/Upright", type: "function" }),
        snippetCompletion("\\mathsf{#{}}", { label: "\\mathsf", detail: "Sans Serif", type: "function" }),
        snippetCompletion("\\mathtt{#{}}", { label: "\\mathtt", detail: "Typewriter", type: "function" })
    ];

    // 3. Common Operations & Relations
    const operationSnippets = [
        createSymbol("cdot", "Dot Product"),
        createSymbol("times", "Cross Product"),
        createSymbol("div", "Division"),
        createSymbol("pm", "Plus-Minus"),
        createSymbol("mp", "Minus-Plus"),
        createSymbol("leq", "Less than or Equal"),
        createSymbol("geq", "Greater than or Equal"),
        createSymbol("neq", "Not Equal"),
        createSymbol("approx", "Approximately"),
        createSymbol("equiv", "Equivalent"),
        createSymbol("propto", "Proportional"),
        createSymbol("sim", "Similar"),
        createSymbol("ll", "Much Less Than"),
        createSymbol("gg", "Much Greater Than"),
        createSymbol("in", "Element Of"),
        createSymbol("subset", "Subset"),
        createSymbol("supset", "Superset"),
        createSymbol("cup", "Union"),
        createSymbol("cap", "Intersection"),
        createSymbol("forall", "For All"),
        createSymbol("exists", "Exists"),
        createSymbol("nabla", "Del / Gradient"),
        createSymbol("partial", "Partial Derivative"),
        createSymbol("infty", "Infinity"),
        createSymbol("hbar", "Reduced Planck Constant"),
        createSymbol("ell", "Cursive l"),
        createSymbol("Im", "Imaginary Part"),
        createSymbol("Re", "Real Part")
    ];

    // 4. Accents & Diacritics
    const accentSnippets = [
        snippetCompletion("\\vec{#{}}", { label: "\\vec", detail: "Vector", type: "function" }),
        snippetCompletion("\\hat{#{}}", { label: "\\hat", detail: "Unit Vector/Operator", type: "function" }),
        snippetCompletion("\\bar{#{}}", { label: "\\bar", detail: "Bar/Mean", type: "function" }),
        snippetCompletion("\\dot{#{}}", { label: "\\dot", detail: "Time Derivative", type: "function" }),
        snippetCompletion("\\ddot{#{}}", { label: "\\ddot", detail: "Second Derivative", type: "function" }),
        snippetCompletion("\\tilde{#{}}", { label: "\\tilde", detail: "Tilde", type: "function" }),
        snippetCompletion("\\overline{#{}}", { label: "\\overline", detail: "Overline", type: "function" })
    ];

    // 5. Structures (Fractions, Roots, Sums)
    const structureSnippets = [
        snippetCompletion("\\frac{#{num}}{#{den}}", { label: "\\frac", detail: "Fraction", type: "keyword" }),
        snippetCompletion("\\sqrt{#{}}", { label: "\\sqrt", detail: "Square Root", type: "keyword" }),
        snippetCompletion("\\sum_{#{i=0}}^{#{n}}", { label: "\\sum", detail: "Summation", type: "keyword" }),
        snippetCompletion("\\prod_{#{i=0}}^{#{n}}", { label: "\\prod", detail: "Product", type: "keyword" }),
        snippetCompletion("\\int_{#{a}}^{#{b}}", { label: "\\int", detail: "Integral", type: "keyword" }),
        snippetCompletion("\\lim_{#{x \\to 0}}", { label: "\\lim", detail: "Limit", type: "keyword" })
    ];

    // 6. Matrices & Environments
    const matrixSnippets = [
        snippetCompletion("\\begin{matrix}\n\t#{}\n\\end{matrix}", { label: "matrix", detail: "Matrix env", type: "class" }),
        snippetCompletion("\\begin{pmatrix}\n\t#{}\n\\end{pmatrix}", { label: "pmatrix", detail: "( Matrix )", type: "class" }),
        snippetCompletion("\\begin{bmatrix}\n\t#{}\n\\end{bmatrix}", { label: "bmatrix", detail: "[ Matrix ]", type: "class" }),
        snippetCompletion("\\begin{vmatrix}\n\t#{}\n\\end{vmatrix}", { label: "vmatrix", detail: "| Determinant |", type: "class" }),
        snippetCompletion("\\begin{cases}\n\t#{}\n\\end{cases}", { label: "cases", detail: "Piecewise", type: "class" }),
        snippetCompletion("\\begin{align}\n\t#{}\n\\end{align}", { label: "align", detail: "Aligned Eq", type: "class" }),
        snippetCompletion("\\left( #{content} \\right)", { label: "\\left(", detail: "Auto-sizing ( )", type: "keyword" }),
        snippetCompletion("\\left[ #{content} \\right]", { label: "\\left[", detail: "Auto-sizing [ ]", type: "keyword" }),
        snippetCompletion("\\left\\{ #{content} \\right\\}", { label: "\\left{", detail: "Auto-sizing { }", type: "keyword" })
    ];

    return [
        ...greekSnippets,
        ...fontSnippets,
        ...operationSnippets,
        ...accentSnippets,
        ...structureSnippets,
        ...matrixSnippets
    ];
};
