"use client";
import dynamic from "next/dynamic";

const Admonition = dynamic(
    () => import("@fluster/webview_utils").then((a) => a.Admonition),
    {
        ssr: false,
    },
);
const Hl = dynamic(() => import("@fluster/webview_utils").then((a) => a.Hl), {
    ssr: false,
});

const Ul = dynamic(() => import("@fluster/webview_utils").then((a) => a.Ul), {
    ssr: false,
});

export { Admonition, Ul, Hl };
