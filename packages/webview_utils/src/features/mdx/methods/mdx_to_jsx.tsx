/* eslint-disable  @typescript-eslint/no-explicit-any -- No need to type library stuff */
import { compile } from "@mdx-js/mdx";
import type { CompileOptions } from "@mdx-js/mdx";
import remarkMath from "remark-math";
import remarkGfm from "remark-gfm";
import rehypeMathjax from "rehype-mathjax/chtml";
import rehypeAutolinkHeadings from "rehype-autolink-headings";
import rehypePrettyCode from "rehype-pretty-code";
import emoji from "remark-emoji";
import rehypeSlug from "rehype-slug";
import rehypeVideo from "rehype-video";
import rehypeMermaid from "rehype-mermaid";

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
    lightCodeTheme?: string;
    darkCodeTheme?: string;
}

const codeThemeStringToThemeName = (input: string): string => {
    const defaultLightTheme = "material-theme-lighter";
    const defaultDarkTheme = "dracula";
    switch (input) {
        case "materialLight":
            return "material-theme-lighter";
        case "solarizedLight":
            return "solarized-light";
        case "solarizedDark":
            return "solarized-dark";
        case "githubLight":
            return "github-light";
        case "aura":
            return "aurora-x";
        case "tokyoNightDay":
            return defaultLightTheme; // Real theme does not exist so sub other light themee.
        case "xcodeLight":
            return defaultLightTheme; // Real theme does not exist so sub other light themee.
        case "dracula":
            return "dracula";
        case "tokyoNight":
            return "tokyo-night";
        case "materialDark":
            return "material-theme-darker";
        case "tokyoNightStorm":
            return defaultDarkTheme;
        case "githubDark":
            return "github-dark";
        case "xcodeDark":
            return defaultDarkTheme;
        default:
            console.log("Could not find theme value for: ", input);
    }
    return input;
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
            rehypeVideo as any,
            {
                test: /\/(.*)(.mp4|.mov|.webm)$/,
                details: false,
            },
        ],
        [rehypeMathjax as any, mathOptions],
        [
            rehypeMermaid,
            {
                strategy: "inline-svg",
                dark: darkMode,
                colorScheme: darkMode ? "dark" : "light",
                mermaidConfig: {
                    // output: "svg",
                    /* theme: { light: 'dark', dark: 'dark' }, */
                    mermaid: {
                        background: "hsl(var(--background))",
                        // themeVariables: mermaidTheme.dark,
                        //         theme: "dark",
                    },
                },
            },
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
        [
            rehypeAutolinkHeadings,
            {
                properties: {
                    className: ["subheading-anchor"],
                    ariaLabel: "Link to section",
                },
            },
        ],
        rehypeSlug,
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
}: {
    content: string;
    lightCodeTheme: string;
    darkCodeTheme: string;
}) => {
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
    return String(res).replaceAll(/classname/g, "className");
};
