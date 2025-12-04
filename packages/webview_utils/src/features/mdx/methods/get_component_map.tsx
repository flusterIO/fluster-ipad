import React from "react";
import { FC } from "react";
import { H1, H2, H3, H4, P } from "@/shared_components/typography/typography";
import { BlockQuote } from "@/shared_components/typography/block_quote";
import { MDXComponents } from "mdx/types";
import { InlineMdxContent } from "../components/inline_mdx_content";
import { Hl } from "../embeddable_mdx_components/hl";
import { MdxInput } from "../embeddable_mdx_components/html/input";
import { AnchorTag } from "../embeddable_mdx_components/html/anchor";
import { AutoInsertedTag } from "../embeddable_mdx_components/auto_inserted/tag";
import { Ul } from "../embeddable_mdx_components/ul";
import { FlusterCitation } from "../embeddable_mdx_components/auto_inserted/fluster_citation";
interface ComponentMapItem {
    /// A regex that will return true if this component is to be included in the component map. This will be prepended with a `<`, so the name should match the component as it will be used in the user's note.
    query: string | string[];
    /* eslint-disable-next-line  -- Need to allow any here */
    component: FC<any>;
    requiresInlineMdx?: boolean;
}

export const componentOverrides: MDXComponents = {
    h1: H1,
    h2: H2,
    h3: H3,
    h4: H4,
    p: P,
    blockquote: BlockQuote,
    mark: Hl,
    /* pre: WrappedCodeBlock, */
    a: AnchorTag,
    /* hr: Hr, */
    input: MdxInput,
    /* img: ImgComponent as any, */
};

const items: ComponentMapItem[] = [
    // -- Utility --
    /* { */
    /*     query: "Div", */
    /*     component: Div, */
    /* }, */
    /* { */
    /*     query: "EqRef", */
    /*     component: EqRef, */
    /* }, */
    /* { */
    /*     query: "TaskList", */
    /*     component: WrappedTaskList, */
    /* }, */
    /* // -- Academic -- */
    /* { */
    /*     query: "Abstract", */
    /*     component: Abstract, */
    /* }, */
    /* //    -- Plots -- */
    /* { */
    /*     query: "Plot", */
    /*     component: PlotBareAss, */
    /* }, */
    /* { */
    /*     query: "ScatterPlot", */
    /*     component: ScatterPlot, */
    /* }, */
    /* { */
    /*     query: "LinePlot", */
    /*     component: LinePlot, */
    /* }, */
    /* { */
    /*     query: "PieChart", */
    /*     component: PieChart, */
    /* }, */
    /* { */
    /*     query: "PieChart", */
    /*     component: PieChart, */
    /* }, */
    /* //     -- 3d -- */
    /* { */
    /*     query: "SurfacePlot", */
    /*     component: SurfacePlot, */
    /* }, */
    /* { */
    /*     query: "LinePlot3d", */
    /*     component: LinePlot3d, */
    /* }, */
    /* { */
    /*     query: "ScatterPlot3d", */
    /*     component: ScatterPlot3d, */
    /* }, */
    /* //    -- Plot Utils -- */
    /* { */
    /*     query: "PlotRef", */
    /*     component: PlotRef, */
    /* }, */
    /* // -- Layout -- */
    /* { */
    /*     query: "Admonition", */
    /*     component: Admonition, */
    /*     requiresInlineMdx: true, */
    /* }, */
    /* { */
    /*     query: "Card", */
    /*     component: EmbeddableCard, */
    /*     requiresInlineMdx: true, */
    /* }, */
    /* { */
    /*     query: "Grid", */
    /*     component: Grid, */
    /* }, */
    /* { */
    /*     query: "GridItem", */
    /*     component: GridItem, */
    /* }, */
    /* { */
    /*     query: "Center", */
    /*     component: Center, */
    /* }, */
    /* // -- Text -- */
    /* { */
    /*     query: "Small", */
    /*     component: Small, */
    /* }, */
    /* // -- Attention Getters -- */
    /* { */
    /*     query: "Hint", */
    /*     component: Hint, */
    /* }, */
    {
        query: "Ul",
        component: Ul,
    },
    {
        query: ["Hl", "HL"],
        component: Hl,
    },
    /* { */
    /*     query: "Quote", */
    /*     component: Blockquote, */
    /* }, */
    /* // -- Media -- */
    /* { */
    /*     query: "Video", */
    /*     component: WrappedVideoComponent, */
    /* }, */
    /* { */
    /*     query: "Audio", */
    /*     component: WrappedAudioComponent, */
    /* }, */
    /* { */
    /*     query: "Image", */
    /*     component: WrappedImage, */
    /* }, */
    /* { */
    /*     query: "Whiteboard", */
    /*     component: EmbeddableWhiteboard, */
    /* }, */
    /* { */
    /*     query: ["Youtube", "YouTube"], */
    /*     component: Youtube, */
    /* }, */
    /* { */
    /*     query: ["Gltf", "Glb"], */
    /*     component: GltfModelView */
    /* }, */
    /* { */
    /*     query: ["Cell", "JupyterCell"], */
    /*     component: JupyterCell, */
    /* }, */
    /* // -- Less Commonly Used Components -- */
    /* { */
    /*     query: "Qr", */
    /*     component: (props: QrCodeProps) => ( */
    /*         <QrCode {...props} getQrCodeSvg={commands.getQrCodeSvg} /> */
    /*     ), */
    /* }, */
    /* { */
    /*     query: "Color", */
    /*     component: ColorSwatch, */
    /* }, */
    /* { */
    /*     query: "ColorPalette", */
    /*     component: ColorPalette, */
    /* }, */
    /* // -- Jupyter -- */
    /* // -- Auto Inserted -- */
    /* { */
    /*     query: "InlineCitation", */
    /*     component: InlineCitation, */
    /* }, */
    {
        query: "AutoInsertedTag",
        component: AutoInsertedTag,
    },
    {
        query: "FlusterCitation",
        component: FlusterCitation,
    },
    /* { */
    /*     query: "DictionaryEntry", */
    /*     component: DictionaryEntry, */
    /* }, */
    /* { */
    /*     query: "EquationTag", */
    /*     component: EquationTag, */
    /* }, */
    /* { */
    /*     query: "VideoTimestampLink", */
    /*     component: VideoTimestampLink, */
    /* }, */
    /* { */
    /*     query: "AudioTimestampLink", */
    /*     component: AudioTimestampLink, */
    /* }, */
    /* // -- Documentation Only -- */
    /* { */
    /*     query: "AppRoute", */
    /*     component: AppRoute, */
    /* }, */
];

export const getComponentMap = (mdxContent: string): MDXComponents => {
    const components: MDXComponents = componentOverrides;
    for (const item of items) {
        for (const query of Array.isArray(item.query) ? item.query : [item.query]) {
            const isIncluded = mdxContent.includes(`<${query}`);
            if (isIncluded) {
                const C = item.component;
                const props = {
                    InlineMdxContent: item.requiresInlineMdx
                        ? InlineMdxContent
                        : undefined,
                };
                components[query] = (_props: object) => <C {...props} {..._props} />;
            }
        }
    }
    return components;
};
