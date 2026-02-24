import { Config } from "tailwindcss/types/config";
import typography from "@tailwindcss/typography";
import containerQueries from "@tailwindcss/container-queries";
import path from "path";

if (!process.env.FLUSTER_IOS_ROOT) {
    console.log(
        `Cannot continue without FLUSTER_IOS_ROOT environment variable set to the root of the workspace directory.`,
    );
}

const flattenColorPalette = (colors: any): any =>
    Object.assign(
        {},
        ...Object.entries(
            colors !== null && colors !== void 0 ? colors : {},
        ).flatMap(([color, values]: any) =>
            typeof values == "object"
                ? Object.entries(flattenColorPalette(values)).map(([number, hex]) => ({
                    [color + (number === "DEFAULT" ? "" : `-${number}`)]: hex,
                }))
                : [
                    {
                        [`${color}`]: values,
                    },
                ],
        ),
    );

export const getBaseTailwindConfig = (
    config: Pick<Config, "content"> & {
        includeWebUtils?: boolean;
        content: string[];
    },
): Config => {
    let content: string[] = config.content;
    if (config.includeWebUtils) {
        content.push(
            path.resolve(
                process.env.FLUSTER_IOS_ROOT!,
                "packages/webview_utils/src/**/*.{ts,tsx,mdx}",
            ),
        );
    }

    return {
        content,
        darkMode: "class",
        safelist: ["prose-base", "prose-lg", "prose-xl", "prose-2xl"],
        theme: {
            extend: {
                colors: () => ({
                    border: "hsl(var(--border))",
                    input: "hsl(var(--input))",
                    ring: "hsl(var(--ring))",
                    background: "hsl(var(--background))",
                    foreground: "hsl(var(--foreground))",
                    primary: {
                        DEFAULT: "hsl(var(--primary))",
                        foreground: "hsl(var(--primary-foreground))",
                    },
                    secondary: {
                        DEFAULT: "hsl(var(--secondary))",
                        foreground: "hsl(var(--secondary-foreground))",
                    },
                    destructive: {
                        DEFAULT: "hsl(var(--destructive))",
                        foreground: "hsl(var(--destructive-foreground))",
                    },
                    muted: {
                        DEFAULT: "hsl(var(--muted))",
                        foreground: "hsl(var(--muted-foreground))",
                    },
                    accent: {
                        DEFAULT: "hsl(var(--accent))",
                        foreground: "hsl(var(--accent-foreground))",
                    },
                    popover: {
                        DEFAULT: "hsl(var(--popover))",
                        foreground: "hsl(var(--popover-foreground))",
                    },
                    card: {
                        DEFAULT: "hsl(var(--card))",
                        foreground: "hsl(var(--card-foreground))",
                    },
                    brand: "hsl(var(--brand))",
                    lightBlue: {
                        "100": "#E0F2FE",
                        "200": "#BAE6FD",
                        "300": "#7DD3FC",
                        "400": "#38BDF8",
                        "500": "#0EA5E9",
                        "600": "#0284C7",
                        "700": "#0369A1",
                        "800": "#075985",
                        "900": "#1E3A8A",
                    },
                    hint: "hsl(var(--fluster-hint))",
                    error: "hsl(var(--fluster-error))",
                    link: "hsl(var(--general-link-color))",
                    emphasisInfo: "hsl(var(--emphasis-info))",
                    emphasisError: "hsl(var(--emphasis-error))",
                    emphasisWarn: "hsl(var(--emphasis-warn))",
                    emphasisSuccess: "hsl(var(--emphasis-success))",
                    emphasisImportant: "hsl(var(--emphasis-important))",
                    emphasisResearch: "hsl(var(--emphasis-research))",
                    emphasisInfoForeground: "hsl(var(--emphasis-info-foreground))",
                    emphasisErrorForeground: "hsl(var(--emphasis-error-foreground))",
                    emphasisWarnForeground: "hsl(var(--emphasis-warn-foreground))",
                    emphasisSuccessForeground: "hsl(var(--emphasis-success-foreground))",
                    emphasisImportantForeground: "hsl(var(--emphasis-important-foreground))",
                    emphasisResearchForeground: "hsl(var(--emphasis-research-foreground))",
                    // TODO: Add these colors back in and make them user modifable.
                    // fluster_info: "hsl(var(--fluster-info))",
                    // fluster_error: "hsl(var(--fluster-error))",
                    // fluster_note: "hsl(var(--fluster-note))",
                    // fluster_tip: "hsl(var(--fluster-tip))",
                    // fluster_faq: "hsl(var(--fluster-faq))",
                    // fluster_practice: "hsl(var(--fluster-practice))",
                    // fluster_abstract: "hsl(var(--fluster-abstract))",
                    // fluster_todo: "hsl(var(--fluster-todo))",
                    // fluster_success: "hsl(var(--fluster-success))",
                    // fluster_warn: "hsl(var(--fluster-warn))",
                    // fluster_fail: "hsl(var(--fluster-fail))",
                    // fluster_example: "hsl(var(--fluster-example))",
                    // fluster_quote: "hsl(var(--fluster-quote))",
                    // fluster_cite: "hsl(var(--fluster-cite))",
                    // fluster_equation: "hsl(var(--fluster-equation))",
                    // fluster_definition: "hsl(var(--fluster-definition))",
                    // fluster_important: "hsl(var(--fluster-important))",
                }),
                maxWidth: {
                    content: "1440px",
                },
                typography: ({ theme }: { theme: (val: string) => void }) => ({
                    DEFAULT: {
                        css: {
                            "--tw-prose-invert-body": theme("colors.gray[200]"),
                            "--tw-prose-invert-headings": "hsl(var(--foreground))",
                            pre: {
                                "&:not([data-language])": {
                                    backgroundColor: "hsl(var(--background))",
                                },
                            },
                        },
                    },
                }),
            },
        },
        plugins: [
            typography,
            containerQueries,
            function addVariablesForColors({ addBase, theme }: any) {
                const allColors = flattenColorPalette(theme("colors"));
                const newVars = Object.fromEntries(
                    Object.entries(allColors).map(([key, val]) => [`--${key}`, val]),
                );
                addBase({
                    ":root": newVars,
                });
            },
        ],
    } satisfies Config;
};
