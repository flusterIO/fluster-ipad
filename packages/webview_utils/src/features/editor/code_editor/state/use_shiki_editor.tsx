import { useEffect, useState, useMemo } from 'react';
import { EditorView, Decoration, ViewPlugin, ViewUpdate } from '@codemirror/view';
import { RangeSetBuilder } from '@codemirror/state';
import { BundledLanguage, BundledTheme, createHighlighter, Highlighter } from 'shiki';

// 1. The CodeMirror Plugin
const shikiPlugin = (highlighter: Highlighter, theme: string, lang: BundledLanguage) => {
    return ViewPlugin.fromClass(
        class {
            decorations: any;

            constructor(view: EditorView) {
                this.decorations = this.buildDecorations(view);
            }

            update(update: ViewUpdate) {
                // Only re-highlight if the doc changed or the viewport changed
                if (update.docChanged || update.viewportChanged) {
                    this.decorations = this.buildDecorations(update.view);
                }
            }

            buildDecorations(view: EditorView) {
                const builder = new RangeSetBuilder<Decoration>();
                const doc = view.state.doc.toString();

                // Shiki tokenizes the entire document (Caution: Performance heavy on large files)
                const tokenLines = highlighter.codeToTokens(doc, {
                    lang,
                    theme
                }).tokens;

                let pos = 0;

                for (const line of tokenLines) {
                    for (const token of line) {
                        // Add a decoration for each token
                        builder.add(
                            pos,
                            pos + token.content.length,
                            Decoration.mark({
                                attributes: { style: `color: ${token.color}` }
                            })
                        );
                        pos += token.content.length;
                    }
                    // Account for the newline character Shiki swallows
                    pos += 1;
                }

                return builder.finish();
            }
        },
        {
            decorations: (v) => v.decorations,
        }
    );
};

// 2. The React Hook
export function useShiki({ theme = 'nord', lang = 'javascript' }: {
    theme: BundledTheme
    lang: BundledLanguage
}) {
    const [highlighter, setHighlighter] = useState<Highlighter | null>(null);

    useEffect(() => {
        let mounted = true;

        // Initialize Shiki (Async)
        createHighlighter({
            themes: [theme],
            langs: [lang],
        }).then((h) => {
            if (mounted) setHighlighter(h);
        });

        return () => { mounted = false; };
    }, [theme, lang]);

    // Create the extension only when the highlighter is ready
    const shikiExtension = useMemo(() => {
        if (!highlighter) return [];
        return shikiPlugin(highlighter, theme, lang);
    }, [highlighter, theme, lang]);

    return { highlighter, shikiExtension };
}
