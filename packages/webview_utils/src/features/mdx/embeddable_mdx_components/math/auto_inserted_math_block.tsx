import React, { useId, useMemo } from 'react';
import { mathjax } from 'mathjax-full/js/mathjax.js';
import { TeX } from 'mathjax-full/js/input/tex.js';
import { CHTML } from 'mathjax-full/js/output/chtml.js';
import { liteAdaptor } from 'mathjax-full/js/adaptors/liteAdaptor.js';
import { RegisterHTMLHandler } from 'mathjax-full/js/handlers/html.js';
import { AllPackages as allPackages } from 'mathjax-full/js/input/tex/AllPackages.js'
import { connect } from 'react-redux';
import { type GlobalWebviewStateDeepNullable } from '#/webview_global_state/cross_language_state_types';
import { type LiteElement } from 'mathjax-full/js/adaptors/lite/Element';
import consola from 'consola';
import { type MathDocument } from 'mathjax-full/js/core/MathDocument';
import { type LiteText } from 'mathjax-full/js/adaptors/lite/Text';
import { type LiteDocument } from 'mathjax-full/js/adaptors/lite/Document';
import { type MathData } from '../../../../core/code_gen/typeshare/conundrum';


interface Props {
    data: MathData,
    mathFontUrl: GlobalWebviewStateDeepNullable["math"]["mathjax_font_url"]
}


// 1. Initialize MathJax outside the component so it only runs once
const adaptor = liteAdaptor();
RegisterHTMLHandler(adaptor);

const connector = connect((state: GlobalWebviewStateDeepNullable) => ({
    mathFontUrl: state.math.mathjax_font_url
}))

const tex = new TeX({
    packages: allPackages,
    /* tags: "all", */
    useLabelIds: true,
    processEscapes: true,
    processEnvironments: true
});

export const AutoInsertedMathBlock = connector(({ data, mathFontUrl }: Props) => {
    const { content, display, id, idx } = data;
    const reactId = useId()
    const [htmlString, cssString] = useMemo((): [string, string] => {
        if (!mathFontUrl.length) {
            consola.error("Cannot render math without a font url provided.")
            return ["<span>Something went wrong</span>", ""]
        }
        try {
            const htmlOutput = new CHTML({
                /* fontCache: 'local', */
                fontURL: mathFontUrl,
                adaptiveCSS: true,
                minScale: 0.2,
                scale: 1.2
            });
            const doc: MathDocument<LiteElement, LiteText, LiteDocument> = mathjax.document('', { InputJax: tex, OutputJax: htmlOutput }) as MathDocument<LiteElement, LiteText, LiteDocument>;
            const node: LiteElement = doc.convert(content, { display: typeof display === "boolean" ? display : false }) as LiteElement;
            const res = adaptor.outerHTML(node);
            const styleSheet: LiteElement = htmlOutput.styleSheet(doc as MathDocument<unknown, unknown, unknown>) as LiteElement;
            return [res, (styleSheet.children[0] as { value?: string } | undefined)?.value ?? ""]
        } catch (error) {
            consola.error("MathJax formatting error:", error);
            return [`<span class="text-destructive">Math Error</span>`, ""];
        }
    }, [content, mathFontUrl]);



    if (display) {
        return (
            <div data-conundrum-eq-idx={idx} id={id ? `eq-${id}` : reactId} className="w-full flex flex-col justify-center items-center relative px-6">
                <div
                    dangerouslySetInnerHTML={{ __html: htmlString }}
                    style={{ display: 'block' }}
                />
                <style>
                    {cssString}
                </style>
                {typeof idx === "number" ? (<div className="conundrum-math-label text-sm text-muted-foreground absolute top-[50%] translate-y-[-50%] right-0 font-mono">
                    {idx + 1}
                </div>) : null}
            </div>
        )
    }

    return (
        <>
            <span
                dangerouslySetInnerHTML={{ __html: htmlString }}
                style={{ display: 'inline' }}
            />
            <style>
                {cssString}
            </style>
        </>
    );
});
