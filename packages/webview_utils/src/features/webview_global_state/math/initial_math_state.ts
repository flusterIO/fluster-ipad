import { type MathState, EquationNumberingStrategy } from "@/code_gen/typeshare/fluster_core_utilities";

export const initialMathState: MathState = {
    // TODO: Change this to a cdm for the Katex fonts now that we're using KaTeX. It should be completely unecssary, but it should actually work if needed.
    mathjax_font_url: "https://cdn.jsdelivr.net/npm/mathjax@3/es5/output/chtml/fonts/woff-v2",
    equation_refs: {},
    hide_equation_labels: EquationNumberingStrategy.All
}
