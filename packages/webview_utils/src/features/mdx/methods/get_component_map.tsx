import React from "react";
import { type FC } from "react";
import { H1, H2, H3, H4, P } from "@/shared_components/typography/typography";
import { BlockQuote } from "@/shared_components/typography/block_quote";
import { type MDXComponents } from "mdx/types";
import { MdxInput } from "../embeddable_mdx_components/html/input";
import { AnchorTag } from "../embeddable_mdx_components/html/anchor";
import { Hl } from "../embeddable_mdx_components/hl/hl";
import { InlineMdxContent } from "../components/inline_mdx_content";
import { ErrorBoundary } from "react-error-boundary";
import { InContentErrorReport } from "../error_reporting/in_content_error_component/in_content_error_report";
import { AutoInsertedCodeBlock } from "../embeddable_mdx_components/auto_inserted/auto_inserted_code_block/auto_inserted_code_block";
import { DocumentationComponentName, EmbeddableComponentName } from "../../../core/code_gen/typeshare/conundrum";
import { admonitionComponentNames } from "../embeddable_mdx_components/admonition/admonition_component_config";
import { cardComponentNames } from "../embeddable_mdx_components/card/embeddable_card_component_config";
import { gridComponentNames } from "../embeddable_mdx_components/grid/embeddable_responsive_grid_component_config";
import { utilityContainerComponentNames } from "../embeddable_mdx_components/container/embeddable_utility_container_component_config";
import { ulComponentNames } from "../embeddable_mdx_components/ul/ul_component_config";
import { hlComponentNames } from "../embeddable_mdx_components/hl/hl_component_config";
import { AutoInsertedComponentName } from "../../../core/code_gen/typeshare/conundrum"
import { hrComponentNames } from "../embeddable_mdx_components/hr/hr_component_config";
import { embeddableHintComponentNames } from "../embeddable_mdx_components/hint/hint_component_config";
import { FoundationModelAvailabilityWrapper } from "../../ai/presentation/foundation_model_availability_wrapper";
import { embeddableTabComponentNames } from "../embeddable_mdx_components/tabs/embeddable_tab_config";
import { tabGroupComponentNames } from "../embeddable_mdx_components/tabs/tab_group_component_config";

enum ComponentItemType {
    userInserted,
    autoInserted,
    documentation,
    ai
}

export type ComponentMapItem = {
    componentType: ComponentItemType.autoInserted,
    query: AutoInsertedComponentName[],
    /* eslint-disable-next-line  -- Not worth typing this. */
    importComponent: () => Promise<FC<any>>;
} | {
    componentType: ComponentItemType.userInserted,
    query: readonly EmbeddableComponentName[],
    /* eslint-disable-next-line  -- Not worth typing this. */
    importComponent: () => Promise<FC<any>>;
} | {
    componentType: ComponentItemType.documentation,
    query: DocumentationComponentName[],
    /* eslint-disable-next-line  -- Not worth typing this. */
    importComponent: () => Promise<FC<any>>;
} | {
    componentType: ComponentItemType.ai,
    query: EmbeddableComponentName[],
    /* eslint-disable-next-line  -- Not worth typing this. */
    importComponent: () => Promise<FC<any>>;
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
        importComponent: async () => {
            return import("../embeddable_mdx_components/container/embeddable_utility_container").then((a) => a.EmbeddableUtilityContainer)
        }
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
        /* component: (props) => <Admonition {...props} InlineMdxContent={InlineMdxContent} />, */
        importComponent: async () => {
            const Component = await import("../embeddable_mdx_components/admonition").then((a) => a.Admonition)
            return (p) => <Component {...p} InlineMdxContent={InlineMdxContent} />
        }
    },
    {
        query: hrComponentNames,
        componentType: ComponentItemType.userInserted,
        importComponent: async () => {
            return await import("../embeddable_mdx_components/hr/embedded_hr_with_children").then((a) => a.EmbeddedHrWithChildren)
        }
    },
    {
        query: cardComponentNames,
        componentType: ComponentItemType.userInserted,
        importComponent: async () => {
            const Component = await import("../embeddable_mdx_components/card/embeddable_card").then((a) => a.EmbeddableCard)
            return (p) => <Component {...p} InlineMdxContent={InlineMdxContent} />
        }
    },
    {
        query: gridComponentNames,
        componentType: ComponentItemType.userInserted,
        importComponent: async () => {
            return import("../embeddable_mdx_components/grid/embeddable_responsive_grid").then((a) => a.EmbeddableResponsiveGrid)
        }
    },
    {
        query: tabGroupComponentNames,
        componentType: ComponentItemType.userInserted,
        importComponent: async () => {
            return import("../embeddable_mdx_components/tabs/tab_group").then((a) => a.WrappedEmbeddableTabGroup)
        }
    },
    {
        query: embeddableTabComponentNames,
        componentType: ComponentItemType.userInserted,
        importComponent: async () => {
            return import("../embeddable_mdx_components/tabs/tab_group_tab").then((a) => a.EmbeddableTab)
        }
    },
    /* // -- Text -- */
    /* // -- Attention Getters -- */
    {
        query: embeddableHintComponentNames,
        componentType: ComponentItemType.userInserted,
        importComponent: async () => {
            return await import("../embeddable_mdx_components/hint/embeddable_hint_component").then((a) => a.EmbeddableHintComponent)
        }
    },
    {
        query: ulComponentNames,
        componentType: ComponentItemType.userInserted,
        importComponent: async () => {
            return import("../embeddable_mdx_components/ul/ul").then((a) => a.Ul)
        }
    },
    {
        query: hlComponentNames,
        componentType: ComponentItemType.userInserted,
        importComponent: async () => {
            return import("../embeddable_mdx_components/hl/hl").then((a) => a.Hl)
        }
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
    // -- AI __
    {
        query: [EmbeddableComponentName.AINoteSummary],
        componentType: ComponentItemType.ai,
        importComponent: async () => {
            return import("../../ai/embeddable_components/ai_note_summary/ai_note_summary").then((a) => a.AiNoteSummary)
        }
    },
    // --- AUTO-INSERTED ---
    {
        query: [AutoInsertedComponentName.NoteLink],
        componentType: ComponentItemType.autoInserted,
        importComponent: async () => {
            return import("../embeddable_mdx_components/auto_inserted/note_link").then((a) => a.NoteLink)
        }
    },
    {
        query: [AutoInsertedComponentName.AutoInsertedTag],
        componentType: ComponentItemType.autoInserted,
        importComponent: async () => {
            return import("../embeddable_mdx_components/auto_inserted/tag").then((a) => a.AutoInsertedTag)
        }
    },
    {
        query: [AutoInsertedComponentName.FlusterCitation],
        componentType: ComponentItemType.autoInserted,
        importComponent: async () => {
            return import("../embeddable_mdx_components/auto_inserted/fluster_citation").then((a) => a.FlusterCitation)
        }
    },
    {
        query: [AutoInsertedComponentName.DictionaryEntry],
        componentType: ComponentItemType.autoInserted,
        importComponent: async () => {
            const Comp = await import("../../dictionary/hooks/dictionary_entry").then((a) => a.DictionaryEntryComponent)
            return (p) => <Comp {...p} InlineMdxContent={InlineMdxContent} />
        }
    },
    {
        query: [AutoInsertedComponentName.FlusterAiParsePendingContainer],
        componentType: ComponentItemType.autoInserted,
        importComponent: async () => {
            return import("../../ai/presentation/foundation_model_availability_wrapper").then((a) => a.FoundationModelAvailabilityWrapper)
        }
    },
    // --- Documentation ---
    {
        query: [DocumentationComponentName.InContentDocumentationContainer],
        componentType: ComponentItemType.documentation,
        importComponent: async () => {
            return import("../../in_content_documentation/presentation/in_content_documentation_container").then((a) => a.InContentDocumentationContainer)
        }
    },
    {
        query: [DocumentationComponentName.InContentDocsEmphasisTypeList],
        componentType: ComponentItemType.documentation,
        importComponent: async () => {
            return import("../../in_content_documentation/presentation/in_content_documentation_components/emphasis_typescript_documentation").then((a) => a.InContentDocsEmphasisTypeList)
        }
    },
    {
        query: [DocumentationComponentName.InContentDocsHighlightDemo],
        componentType: ComponentItemType.documentation,
        importComponent: async () => {
            return import("../../in_content_documentation/presentation/in_content_documentation_components/highlight_demo").then((a) => a.InContentHighlightDocsDemo)
        }
    },
    {
        query: [DocumentationComponentName.InContentDocsUnderlineDemo],
        componentType: ComponentItemType.documentation,
        importComponent: async () => {
            return import("../../mdx/embeddable_mdx_components/ul/documentation_underline_demo").then((a) => a.InContentUnderlineDocumentationDemo)
        }
    },
    // --- Auto Inserted ---
    {
        query: [AutoInsertedComponentName.AutoInsertedMarkdownLink],
        componentType: ComponentItemType.autoInserted,
        importComponent: async () => {
            return import("../../mdx/embeddable_mdx_components/auto_inserted/markdown_link").then((a) => a.AutoInsertedMarkdownLink)
        }
    },
    {
        query: [AutoInsertedComponentName.AutoInsertedHeading],
        componentType: ComponentItemType.autoInserted,
        importComponent: async () => {
            return import("../../mdx/embeddable_mdx_components/auto_inserted/heading").then((a) => a.AutoInsertedHeading)
        }
    },
    {
        query: [AutoInsertedComponentName.AutoInsertedBlockMath],
        componentType: ComponentItemType.autoInserted,
        importComponent: async () => {
            return import("../../mdx/embeddable_mdx_components/auto_inserted/block_math").then((a) => a.AutoInsertedBlockMath)
        }
    },
    {
        query: [AutoInsertedComponentName.AutoInsertedInlineMath],
        componentType: ComponentItemType.autoInserted,
        importComponent: async () => {
            return import("../../mdx/embeddable_mdx_components/auto_inserted/inline_math").then((a) => a.AutoInsertedInlineMath)
        }
    },
    {
        query: [AutoInsertedComponentName.AutoInsertedBlockQuote],
        componentType: ComponentItemType.autoInserted,
        importComponent: async () => {
            return import("../../mdx/embeddable_mdx_components/auto_inserted/block_quote").then((a) => a.AutoInsertedBlockQuote)
        }
    }
];

export const getComponentMap = async (mdxContent: string, additionalComponenets: ComponentMapItem[] = []): Promise<MDXComponents> => {
    const components: MDXComponents = componentOverrides;
    const x = [...items, ...additionalComponenets]
    for (const item of x) {
        for (const query of item.query) {
            const isIncluded = mdxContent.includes(`<${query}`);
            if (isIncluded) {
                const C = await item.importComponent()
                components[query] = (_props: object) => {
                    if (item.componentType === ComponentItemType.ai) {
                        return (
                            <ErrorBoundary FallbackComponent={(errorProps) => <InContentErrorReport {...errorProps} componentName={query} />}>
                                <FoundationModelAvailabilityWrapper>
                                    <C {..._props} />
                                </FoundationModelAvailabilityWrapper>

                            </ErrorBoundary>
                        )
                    } else {
                        return (
                            <ErrorBoundary FallbackComponent={(errorProps) => <InContentErrorReport {...errorProps} componentName={query} />}><C {..._props} /></ErrorBoundary>
                        )
                    }
                }
            }
        }
    }
    return components;
};
