import React from "react";
import { FC } from "react";
import { H1, H2, H3, H4, P } from "@/shared_components/typography/typography";
import { BlockQuote } from "@/shared_components/typography/block_quote";
import { MDXComponents } from "mdx/types";
import { MdxInput } from "../embeddable_mdx_components/html/input";
import { AnchorTag } from "../embeddable_mdx_components/html/anchor";
import { AutoInsertedTag } from "../embeddable_mdx_components/auto_inserted/tag";
import { FlusterCitation } from "../embeddable_mdx_components/auto_inserted/fluster_citation";
import { Admonition } from "../embeddable_mdx_components/admonition";
import { DictionaryEntryComponent } from "../../dictionary/hooks/dictionary_entry";
import { NoteLink } from "../embeddable_mdx_components/auto_inserted/note_link";
import { Hl } from "../embeddable_mdx_components/hl/hl";
import { Ul } from "../embeddable_mdx_components/ul/ul";
import { InlineMdxContent } from "../components/inline_mdx_content";
import { ErrorBoundary } from "react-error-boundary";
import { InContentErrorReport } from "../error_reporting/in_content_error_component/in_content_error_report";
import { AutoInsertedCodeBlock } from "../embeddable_mdx_components/auto_inserted/auto_inserted_code_block/auto_inserted_code_block";
import { EmbeddableCard } from "../embeddable_mdx_components/card/embeddable_card";
import { EmbeddableResponsiveGrid } from "../embeddable_mdx_components/grid/embeddable_responsive_grid";
import { EmbeddableUtilityContainer } from "../embeddable_mdx_components/container/embeddable_utility_container";
import { EmbeddableComponentName } from "../../../core/code_gen/typeshare/fluster_core_utilities";
import { admonitionComponentNames } from "../embeddable_mdx_components/admonition/admonition_component_config";
import { cardComponentNames } from "../embeddable_mdx_components/card/embeddable_card_component_config";
import { gridComponentNames } from "../embeddable_mdx_components/grid/embeddable_responsive_grid_component_config";
import { utilityContainerComponentNames } from "../embeddable_mdx_components/container/embeddable_utility_container_component_config";
import { ulComponentNames } from "../embeddable_mdx_components/ul/ul_component_config";
import { hlComponentNames } from "../embeddable_mdx_components/hl/hl_component_config";

enum ComponentItemType {
    userInserted,
    autoInserted,
}

export type ComponentMapItem = {
    componentType: ComponentItemType.autoInserted,
    query: string[],
    /* eslint-disable-next-line  -- Not worth typing this. */
    component: FC<any>
} | {
    componentType: ComponentItemType.userInserted,
    query: readonly EmbeddableComponentName[],
    /* eslint-disable-next-line  -- Not worth typing this. */
    component: FC<any>
};

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
    pre: AutoInsertedCodeBlock,
};

const items: ComponentMapItem[] = [
    // -- Utility --
    {
        query: utilityContainerComponentNames,
        componentType: ComponentItemType.userInserted,
        component: EmbeddableUtilityContainer,
    },
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
    {
        query: admonitionComponentNames,
        componentType: ComponentItemType.userInserted,
        // Required to get around circular import that I still can't find...
        component: (props) => <Admonition {...props} InlineMdxContent={InlineMdxContent} />,
    },
    {
        query: cardComponentNames,
        componentType: ComponentItemType.userInserted,
        component: (props) => <EmbeddableCard {...props} InlineMdxContent={InlineMdxContent} />,
    },
    {
        query: gridComponentNames,
        componentType: ComponentItemType.userInserted,
        component: EmbeddableResponsiveGrid,
    },
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
        query: ulComponentNames,
        componentType: ComponentItemType.userInserted,
        component: Ul,
    },
    {
        query: hlComponentNames,
        componentType: ComponentItemType.userInserted,
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
        query: ["NoteLink"],
        componentType: ComponentItemType.autoInserted,
        component: NoteLink
    },
    {
        query: ["AutoInsertedTag"],
        componentType: ComponentItemType.autoInserted,
        component: AutoInsertedTag,
    },
    {
        query: ["FlusterCitation"],
        componentType: ComponentItemType.autoInserted,
        component: FlusterCitation,
    },
    {
        query: ["DictionaryEntry"],
        componentType: ComponentItemType.autoInserted,
        // Required to get around circular import that I still can't find...
        component: (props) => <DictionaryEntryComponent {...props} InlineMdxContent={InlineMdxContent} />,
    },
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

export const getComponentMap = (mdxContent: string, additionalComponenets: ComponentMapItem[] = []): MDXComponents => {
    const components: MDXComponents = componentOverrides;
    const x = [...items, ...additionalComponenets]
    for (const item of x) {
        for (const query of item.query) {
            const isIncluded = mdxContent.includes(`<${query}`);
            if (isIncluded) {
                const C = item.component;
                components[query] = (_props: object) => <ErrorBoundary FallbackComponent={(errorProps) => <InContentErrorReport {...errorProps} componentName={query} />}><C {..._props} /></ErrorBoundary>;
            }
        }
    }
    return components;
};
