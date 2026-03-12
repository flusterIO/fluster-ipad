/* eslint-disable  @typescript-eslint/no-explicit-any -- No need to type library stuff */
import { compile } from "@mdx-js/mdx";
import type { CompileOptions } from "@mdx-js/mdx";
import remarkMath from "remark-math";
import remarkGfm from "remark-gfm";
import rehypeMathjax from "rehype-mathjax/chtml";
import rehypePrettyCode from "rehype-pretty-code";
import emoji from "remark-emoji";
import rehypeMermaid, { type RehypeMermaidOptions } from "rehype-mermaid";
import withSlugs from "rehype-slug";
import withCustomIds from "remark-custom-header-id"
import withToc, { type Toc } from "@stefanprobst/rehype-extract-toc";
import { CodeEditorTheme } from "@/code_gen/typeshare/fluster_core_utilities";
import { type BundledTheme } from "shiki";

export const mathOptions = {
    tex: {
        // packages: [],
        tags: "all", // "all" | "ams" (ams breaks EqRef component, unless can find other way to force label creation.),
        useLabelIds: true,
        processEscapes: true,
        processEnvironments: true,
    },
    chtml: {
        // FIXME: Figure out a way to point this to the proper location on the user's device if possible to make math render properly without wifi, alternatively, move away from the chtml method.
        fontURL:
            "https://cdn.jsdelivr.net/npm/mathjax@3/es5/output/chtml/fonts/woff-v2",
        // TODO: Fix this. Turn this fontURL to the new path that will hopefully work...
        /* fontURL: ResourceRoutes.mathjaxFonts, */
        // process.env.NODE_ENV === "development"
        //     ? ResourceRoutes.mathjaxFonts
        //     : "https://cdn.jsdelivr.net/npm/mathjax@3/es5/output/chtml/fonts/woff-v2",
        adaptiveCSS: true,
    },
};

interface RehypePluginProps {
    lightCodeTheme?: CodeEditorTheme;
    darkCodeTheme?: CodeEditorTheme;
}

const codeThemeStringToThemeName = (input: CodeEditorTheme): BundledTheme => {
    switch (input) {
        case CodeEditorTheme.MaterialLight: {
            return "material-theme-lighter"
        }
        case CodeEditorTheme.SolarizedLight: {
            return "solarized-light"
        }
        case CodeEditorTheme.SolarizedDark: {
            return "solarized-dark"
        }
        case CodeEditorTheme.GithubLight: {
            return "github-light"
        }
        case CodeEditorTheme.Aura: {
            return "aurora-x"
        }
        case CodeEditorTheme.TokyoNightDay: {
            return "tokyo-night"
        }
        case CodeEditorTheme.XcodeLight: {
            return "github-light-high-contrast"
        }
        case CodeEditorTheme.Dracula: {
            return "dracula"
        }
        case CodeEditorTheme.TokyoNight: {
            return "tokyo-night"
        }
        case CodeEditorTheme.MaterialDark: {
            return "material-theme-darker"
        }
        case CodeEditorTheme.TokyoNightStorm: {
            return "tokyo-night"
        }
        case CodeEditorTheme.GithubDark: {
            return "github-dark"
        }
        case CodeEditorTheme.XcodeDark: {
            return "dark-plus"
        }
    }
};

const rehypePlugins = ({
    lightCodeTheme,
    darkCodeTheme,
}: RehypePluginProps): CompileOptions["rehypePlugins"] => {
    const darkMode = document
        .getElementById("webview-container")
        ?.classList.contains("dark");
    // let shikiTransformers = await getShikiTransformers(config)
    return [
        /* TODO: Add an embeded video component for this rehypeVideo that then utilizes the existing video element. */
        [
            withSlugs
        ],
        [
            withToc,
        ],
        [rehypeMathjax as any, mathOptions],
        [
            rehypeMermaid,
            {
                strategy: "inline-svg",
                dark: darkMode ? true : undefined,
                colorScheme: darkMode ? "dark" : "light",
                mermaidConfig: {
                    darkMode,
                    theme: "base",
                    themeVariables: {
                        background: "hsl(var(--background))"
                    },
                },
            } satisfies RehypeMermaidOptions,
        ],
        [
            rehypePrettyCode as any,
            {
                keepBackground: true,
                theme: {
                    light: lightCodeTheme
                        ? codeThemeStringToThemeName(lightCodeTheme)
                        : "material-theme-lighter",
                    dark: darkCodeTheme
                        ? codeThemeStringToThemeName(darkCodeTheme)
                        : "dracula",
                },
                onVisitLine(node: any) {
                    if (node.children.length === 0) {
                        node.children = [{ type: "text", value: " " }];
                    }
                },
                onVisitHighlightedLine(node: any) {
                    node.properties.className.push("line--highlighted");
                },
                onVisitHighlightedWord(node: any) {
                    node.properties.className = ["word--highlighted"];
                },
                // transformers: shikiTransformers,
                defaultLang: {
                    block: "python",
                    inline: "zsh",
                },
            },
        ],
        /* [ */
        /*     rehypeImgSize, */
        /*     { dir: "" } */
        /* ], */
    ];
};

const remarkPlugins = (): /* config?: AppConfigSchemaOutput, */
    CompileOptions["remarkPlugins"] => {
    return [remarkMath, remarkGfm, emoji, withCustomIds];
};

export const parseMdxString = async ({
    content,
    lightCodeTheme,
    darkCodeTheme,
}: {
    content: string;
    lightCodeTheme: CodeEditorTheme;
    darkCodeTheme: CodeEditorTheme;
}): Promise<{ value: string, toc?: Toc }> => {
    const res = await compile(content, {
        outputFormat: "function-body",
        remarkPlugins: remarkPlugins(),
        rehypePlugins: rehypePlugins({
            lightCodeTheme,
            darkCodeTheme,
        }),
        // development: process.env.NODE_ENV === "development",
        /* baseUrl: import.meta.url */
    });
    console.log("Parsed Mdx with new props: ", res)
    /* res. */
    return { value: String(res.value).replaceAll(/classname/g, "className"), toc: res.data.toc }
};
