import "@fluster/webview_utils/styles.css";
import "@conundrum/ts/styles.css";
import "../core/styles/globals.scss";
import localFont from "next/font/local";
import { GoogleAnalytics } from "@next/third-parties/google";
import Head from "next/head";
import { Analytics } from "@vercel/analytics/next";
import { type Metadata } from "next";

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
    title: "Fluster",
    description: "Academic note taking for the modern researcher.",
};

export default function RootLayout({
    children,
}: Readonly<{
    children: React.ReactNode;
}>) {
    return (
        <html lang="en" className={`${appFont.variable} dark`}>
            <Head>
                <title>Fluster</title>
                <meta
                    name="description"
                    content="Free & open source academic note taking framework."
                />
                <meta property="og:title" content="Fluster" key="title" />
                <meta
                    property="og:description"
                    content="Fluster is your brain's free & open source presentation layer for modern academic note taking."
                />
                <meta name="twitter:card" content="summary_large_image" />
            </Head>
            <Analytics />
            <body className={`antialiased background dark`}>
                <div>{children}</div>
            </body>
            <GoogleAnalytics gaId="G-Y02PEY1GJZ" />
        </html>
    );
}
