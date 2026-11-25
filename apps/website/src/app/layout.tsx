import "@fluster/webview_utils/styles.css";
import "@conundrum/ts/styles.css";
import "../core/styles/globals.scss";
import localFont from "next/font/local";
import { GoogleAnalytics } from "@next/third-parties/google";
import { Analytics } from "@vercel/analytics/next";
import { type Metadata, type Viewport } from "next";
import { JsonLd } from "#/core/seo/json_ld";
import { seo, siteUrl } from "#/core/seo/site";
import {
    conundrumJsonLd,
    organizationJsonLd,
    softwareApplicationJsonLd,
    websiteJsonLd,
} from "#/core/seo/structured_data";

const appFont = localFont({
    variable: "--ulld-app-font",
    src: [
        {
            path: "../../assets/appFont/DM_Sans/static/DMSans-Thin.ttf",
            weight: "100",
            style: "normal",
        },
        {
            path: "../../assets/appFont/DM_Sans/static/DMSans-Bold.ttf",
            weight: "700",
            style: "normal",
        },
        {
            path: "../../assets/appFont/DM_Sans/static/DMSans-Light.ttf",
            weight: "300",
            style: "normal",
        },
        {
            path: "../../assets/appFont/DM_Sans/static/DMSans-Regular.ttf",
            weight: "400",
            style: "normal",
        },
        {
            path: "../../assets/appFont/DM_Sans/static/DMSans-Italic.ttf",
            weight: "400",
            style: "italic",
        },
        {
            path: "../../assets/appFont/DM_Sans/static/DMSans-SemiBold.ttf",
            weight: "600",
            style: "normal",
        },
        {
            path: "../../assets/appFont/DM_Sans/static/DMSans-ExtraBold.ttf",
            weight: "800",
            style: "normal",
        },
    ],
    display: "swap",
});

export const metadata: Metadata = {
    metadataBase: new URL(siteUrl),
    title: { default: seo.title, template: seo.titleTemplate },
    description: seo.description,
    keywords: [...seo.keywords],
    applicationName: seo.siteName,
    authors: [{ name: seo.author }],
    creator: seo.author,
    openGraph: {
        type: "website",
        siteName: seo.siteName,
        title: seo.title,
        description: seo.description,
        locale: "en_US",
    },
    twitter: {
        card: "summary_large_image",
        site: seo.twitterHandle,
        creator: seo.twitterHandle,
        title: seo.title,
        description: seo.description,
    },
    robots: {
        index: true,
        follow: true,
        googleBot: {
            index: true,
            follow: true,
            "max-image-preview": "large",
            "max-snippet": -1,
            "max-video-preview": -1,
        },
    },
    category: "education",
};

export const viewport: Viewport = {
    themeColor: "#000000",
    colorScheme: "dark",
};

export default function RootLayout({
    children,
}: Readonly<{
    children: React.ReactNode;
}>) {
    return (
        <html lang="en" className={`${appFont.variable} dark`}>
            <Analytics />
            <body className={`antialiased background dark`}>
                <JsonLd
                    data={[
                        organizationJsonLd,
                        websiteJsonLd,
                        softwareApplicationJsonLd,
                        conundrumJsonLd,
                    ]}
                />
                <div>{children}</div>
            </body>
            <GoogleAnalytics gaId="G-Y02PEY1GJZ" />
        </html>
    );
}
