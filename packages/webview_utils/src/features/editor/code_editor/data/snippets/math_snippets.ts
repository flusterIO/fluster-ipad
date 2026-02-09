import { CompletionSections, SnippetDefaultType } from "#/mdx/embeddable_mdx_components/embeddable_component_config";
import { Completion, snippetCompletion } from "@codemirror/autocomplete";
import { SnippetItem, SnippetStrategy } from "./snippet_types";

export const getMathSnippets = (base?: string): SnippetItem[] => {
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
    const createSymbol = (name: string, detail: string = "Greek Letter", type: SnippetDefaultType = SnippetDefaultType.keyword) => ({
        label: `\\${name}`,
        detail,
        type,
        apply: `\\${name}`,
        section: CompletionSections.math
    });

    // 1. Greek Letters
    const greekSnippets = [
        ...greekLower.map(g => createSymbol(g, "Greek Lower")),
        ...greekUpper.map(g => createSymbol(g, "Greek Upper")),
        ...greekVars.map(g => createSymbol(g, "Greek Variation"))
    ];

    // 2. Common Math Fonts
    const fontSnippets = [
        snippetCompletion("\\mathbb{#{}}", { label: "\\mathbb", detail: "Blackboard Bold (R, Z, C)", type: SnippetDefaultType.constant, section: CompletionSections.math }),
        snippetCompletion("\\mathcal{#{}}", { label: "\\mathcal", detail: "Calligraphic", type: SnippetDefaultType.constant, section: CompletionSections.math }),
        snippetCompletion("\\mathfrak{#{}}", { label: "\\mathfrak", detail: "Fraktur", type: SnippetDefaultType.constant, section: CompletionSections.math }),
        snippetCompletion("\\mathbf{#{}}", { label: "\\mathbf", detail: "Bold Face", type: SnippetDefaultType.constant, section: CompletionSections.math }),
        snippetCompletion("\\mathrm{#{}}", { label: "\\mathrm", detail: "Roman/Upright", type: SnippetDefaultType.constant, section: CompletionSections.math }),
        snippetCompletion("\\mathsf{#{}}", { label: "\\mathsf", detail: "Sans Serif", type: SnippetDefaultType.constant, section: CompletionSections.math }),
        snippetCompletion("\\mathtt{#{}}", { label: "\\mathtt", detail: "Typewriter", type: SnippetDefaultType.constant, section: CompletionSections.math })
    ];

    // 3. Common Operations & Relations
    const operationSnippets = [
        createSymbol("cdot", "Dot Product", SnippetDefaultType.variable),
        createSymbol("times", "Cross Product", SnippetDefaultType.variable),
        createSymbol("div", "Division", SnippetDefaultType.variable),
        createSymbol("pm", "Plus-Minus", SnippetDefaultType.variable),
        createSymbol("mp", "Minus-Plus", SnippetDefaultType.variable),
        createSymbol("leq", "Less than or Equal", SnippetDefaultType.variable),
        createSymbol("geq", "Greater than or Equal", SnippetDefaultType.variable),
        createSymbol("neq", "Not Equal", SnippetDefaultType.variable),
        createSymbol("approx", "Approximately", SnippetDefaultType.variable),
        createSymbol("equiv", "Equivalent", SnippetDefaultType.variable),
        createSymbol("propto", "Proportional", SnippetDefaultType.variable),
        createSymbol("sim", "Similar", SnippetDefaultType.variable),
        createSymbol("ll", "Much Less Than", SnippetDefaultType.variable),
        createSymbol("gg", "Much Greater Than", SnippetDefaultType.variable),
        createSymbol("in", "Element Of", SnippetDefaultType.variable),
        createSymbol("subset", "Subset", SnippetDefaultType.variable),
        createSymbol("supset", "Superset", SnippetDefaultType.variable),
        createSymbol("cup", "Union", SnippetDefaultType.variable),
        createSymbol("cap", "Intersection", SnippetDefaultType.variable),
        createSymbol("forall", "For All", SnippetDefaultType.variable),
        createSymbol("exists", "Exists", SnippetDefaultType.variable),
        createSymbol("nabla", "Del / Gradient", SnippetDefaultType.variable),
        createSymbol("partial", "Partial Derivative", SnippetDefaultType.variable),
        createSymbol("infty", "Infinity", SnippetDefaultType.variable),
        createSymbol("hbar", "Reduced Planck Constant", SnippetDefaultType.variable),
        createSymbol("ell", "Cursive l", SnippetDefaultType.variable),
        createSymbol("Im", "Imaginary Part", SnippetDefaultType.variable),
        createSymbol("Re", "Real Part", SnippetDefaultType.variable),
    ];

    // 4. Accents & Diacritics
    const accentSnippets = [
        snippetCompletion("\\vec{#{}}", { label: "\\vec", detail: "Vector", type: "function", section: CompletionSections.math }),
        snippetCompletion("\\hat{#{}}", { label: "\\hat", detail: "Unit Vector/Operator", type: "function", section: CompletionSections.math }),
        snippetCompletion("\\bar{#{}}", { label: "\\bar", detail: "Bar/Mean", type: "function", section: CompletionSections.math }),
        snippetCompletion("\\dot{#{}}", { label: "\\dot", detail: "Time Derivative", type: "function", section: CompletionSections.math }),
        snippetCompletion("\\ddot{#{}}", { label: "\\ddot", detail: "Second Derivative", type: "function", section: CompletionSections.math }),
        snippetCompletion("\\tilde{#{}}", { label: "\\tilde", detail: "Tilde", type: "function", section: CompletionSections.math }),
        snippetCompletion("\\overline{#{}}", { label: "\\overline", detail: "Overline", type: "function", section: CompletionSections.math })
    ];

    // 5. Structures (Fractions, Roots, Sums)
    const structureSnippets = [
        snippetCompletion("\\frac{#{num}}{#{den}}", { label: "\\frac", detail: "Fraction", type: SnippetDefaultType.variable, section: CompletionSections.math }),
        snippetCompletion("\\sqrt{#{}}", { label: "\\sqrt", detail: "Square Root", type: SnippetDefaultType.variable, section: CompletionSections.math }),
        snippetCompletion("\\sum_{#{i=0}}^{#{n}}", { label: "\\sum", detail: "Summation", type: SnippetDefaultType.variable, section: CompletionSections.math }),
        snippetCompletion("\\prod_{#{i=0}}^{#{n}}", { label: "\\prod", detail: "Product", type: SnippetDefaultType.variable, section: CompletionSections.math }),
        snippetCompletion("\\int_{#{a}}^{#{b}}", { label: "\\int", detail: "Integral", type: SnippetDefaultType.variable, section: CompletionSections.math }),
        snippetCompletion("\\lim_{#{x \\to 0}}", { label: "\\lim", detail: "Limit", type: SnippetDefaultType.variable, section: CompletionSections.math })
    ];

    // 6. Matrices & Environments
    const matrixSnippets = [
        snippetCompletion("\\begin{matrix}\n\t#{}\n\\end{matrix}", { label: "matrix", detail: "Matrix env", type: SnippetDefaultType.class, section: CompletionSections.math }),
        snippetCompletion("\\begin{pmatrix}\n\t#{}\n\\end{pmatrix}", { label: "pmatrix", detail: "( Matrix )", type: SnippetDefaultType.class, section: CompletionSections.math }),
        snippetCompletion("\\begin{bmatrix}\n\t#{}\n\\end{bmatrix}", { label: "bmatrix", detail: "[ Matrix ]", type: SnippetDefaultType.class, section: CompletionSections.math }),
        snippetCompletion("\\begin{vmatrix}\n\t#{}\n\\end{vmatrix}", { label: "vmatrix", detail: "| Determinant |", type: SnippetDefaultType.class, section: CompletionSections.math }),
        snippetCompletion("\\begin{cases}\n\t#{}\n\\end{cases}", { label: "cases", detail: "Piecewise", type: SnippetDefaultType.class, section: CompletionSections.math }),
        snippetCompletion("\\begin{align}\n\t#{}\n\\end{align}", { label: "align", detail: "Aligned Eq", type: SnippetDefaultType.class, section: CompletionSections.math }),
        snippetCompletion("\\left( #{content} \\right)", { label: "\\left(", detail: "Auto-sizing ( )", type: SnippetDefaultType.variable, section: CompletionSections.math }),
        snippetCompletion("\\left[ #{content} \\right]", { label: "\\left[", detail: "Auto-sizing [ ]", type: SnippetDefaultType.variable, section: CompletionSections.math }),
        snippetCompletion("\\left\\{ #{content} \\right\\}", { label: "\\left{", detail: "Auto-sizing { }", type: SnippetDefaultType.variable, section: CompletionSections.math })
    ];

    const mathBlockSnippets = [
        snippetCompletion("$$\n#{e=mc^2}\n$$", {
            label: "math",
            type: SnippetDefaultType.variable,
            detail: "Open a math block",
            section: CompletionSections.math
        }),
        snippetCompletion("$#{e=mc^2}$", {
            label: "inline-math",
            type: SnippetDefaultType.variable,
            detail: "Open an inline math block",
            section: CompletionSections.math
        }),
    ]

    const withLeadingCharacters = base ? [
        snippetCompletion(`${base}^{#{power}}`, {
            label: `${base}**`,
            section: CompletionSections.math,
            detail: "Latex superscript",
            type: SnippetDefaultType.property
        })
    ] : [];

    return [
        ...greekSnippets.map((n) => {
            return {
                strategy: SnippetStrategy.noLeadingText,
                completion: n
            }
        }),
        ...fontSnippets.map((n) => {
            return {
                strategy: SnippetStrategy.noLeadingText,
                completion: n
            }
        }),
        ...operationSnippets.map((n) => {
            return {
                strategy: SnippetStrategy.noLeadingText,
                completion: n
            }
        }),
        ...accentSnippets.map((n) => {
            return {
                strategy: SnippetStrategy.noLeadingText,
                completion: n
            }
        }),
        ...structureSnippets.map((n) => {
            return {
                strategy: SnippetStrategy.noLeadingText,
                completion: n
            }
        }),
        ...matrixSnippets.map((n) => {
            return {
                strategy: SnippetStrategy.noLeadingText,
                completion: n
            }
        }),
        ...mathBlockSnippets.map((n) => {
            return {
                strategy: SnippetStrategy.noLeadingText,
                completion: n
            }
        }),
        ...withLeadingCharacters.map((n) => {
            return {
                strategy: SnippetStrategy.leadingAnyCharacter,
                completion: n
            }
        })
    ];
};
