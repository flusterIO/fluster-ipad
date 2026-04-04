/* eslint-disable  @typescript-eslint/no-explicit-any -- No need to type library stuff */
import { compile } from "@mdx-js/mdx";
import type { CompileOptions } from "@mdx-js/mdx";
import remarkMath from "remark-math";
import remarkGfm from "remark-gfm";
import rehypeMathjax from "rehype-mathjax/chtml";
import rehypePrettyCode from "rehype-pretty-code";
import emoji from "remark-emoji";
import rehypeMermaid, { type RehypeMermaidOptions } from "rehype-mermaid";
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
        adaptiveCSS: true,
    },
};

interface RehypePluginProps {
    lightCodeTheme?: CodeEditorTheme;
    darkCodeTheme?: CodeEditorTheme;

    mathjaxFontUrl: string
}


/**
 * These are the default theme mappings. For the editor theme as the ket, the code block theme is what is returned.
 * I did my best to match everything, but I'm a s**t designer.
 *
 * The only reason they're not more customizable is because CodeMirror relies on Lexers while the code-blocks are using 
 * Shiki that supports many, _many_ more themes. When I have time I'll do what I can to move the editor over to Shiki, if at all possible.
 */
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
            return "one-dark-pro"
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
    mathjaxFontUrl
}: RehypePluginProps): CompileOptions["rehypePlugins"] => {
    const darkMode = document
        .getElementById("webview-container")
        ?.classList.contains("dark");
    // let shikiTransformers = await getShikiTransformers(config)
    return [
        /* TODO: Add an embeded video component for this rehypeVideo that then utilizes the existing video element. */
        [rehypeMathjax as any, {
            ...mathOptions,
            chtml: {
                ...mathOptions.chtml,
                fontURL: mathjaxFontUrl
            }
        }],
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
    return [remarkMath, remarkGfm, emoji];
};

export const parseMdxString = async ({
    content,
    lightCodeTheme,
    darkCodeTheme,
    mathjaxFontUrl
}: {
    content: string;
    lightCodeTheme: CodeEditorTheme;
    darkCodeTheme: CodeEditorTheme;
    mathjaxFontUrl: string
}): Promise<string> => {
    const res = await compile(content, {
        outputFormat: "function-body",
        remarkPlugins: remarkPlugins(),
        rehypePlugins: rehypePlugins({
            lightCodeTheme,
            darkCodeTheme,
            mathjaxFontUrl
        }),
        // development: process.env.NODE_ENV === "development",
        /* baseUrl: import.meta.url */
    });
    /* res. */
    return String(res.value).replaceAll(/classname/g, "className")
};
