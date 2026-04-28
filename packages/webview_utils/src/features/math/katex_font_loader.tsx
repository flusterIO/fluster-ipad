import { type GlobalWebviewStateDeepNullable } from '#/webview_global_state/cross_language_state_types'
import { withTrailingSlash } from '@/utils/string_utils'
import React, { type ReactNode } from 'react'
import { connect } from 'react-redux'

const connector = connect((state: GlobalWebviewStateDeepNullable) => ({
    fontUrl: state.math.mathjax_font_url
}))

interface KatexFontLoaderProps {
    fontUrl: GlobalWebviewStateDeepNullable["math"]["mathjax_font_url"]
}

export const KatexFontLoader = connector((props: KatexFontLoaderProps): ReactNode => {
    if (!props.fontUrl) {
        return null
    }
    const formattedUrl = withTrailingSlash(props.fontUrl);
    return (
        <style>{`
@font-face {
    font-family: "KaTeX_AMS";
    src: url("${formattedUrl}KaTeX_AMS-Regular.woff2") format("woff2"), url("${formattedUrl}KaTeX_AMS-Regular.woff") format("woff"), url("${formattedUrl}KaTeX_AMS-Regular.ttf") format("truetype");
    font-weight: normal;
    font-style: normal;
    font-display: block;
}

@font-face {
    font-family: "KaTeX_Caligraphic";
    src: url("${formattedUrl}KaTeX_Caligraphic-Bold.woff2") format("woff2"), url("${formattedUrl}KaTeX_Caligraphic-Bold.woff") format("woff"), url("${formattedUrl}KaTeX_Caligraphic-Bold.ttf") format("truetype");
    font-weight: bold;
    font-style: normal;
    font-display: block;
}

@font-face {
    font-family: "KaTeX_Caligraphic";
    src: url("${formattedUrl}KaTeX_Caligraphic-Regular.woff2") format("woff2"), url("${formattedUrl}KaTeX_Caligraphic-Regular.woff") format("woff"), url("${formattedUrl}KaTeX_Caligraphic-Regular.ttf") format("truetype");
    font-weight: normal;
    font-style: normal;
    font-display: block;
}

@font-face {
    font-family: "KaTeX_Fraktur";
    src: url("${formattedUrl}KaTeX_Fraktur-Bold.woff2") format("woff2"), url("${formattedUrl}KaTeX_Fraktur-Bold.woff") format("woff"), url("${formattedUrl}KaTeX_Fraktur-Bold.ttf") format("truetype");
    font-weight: bold;
    font-style: normal;
    font-display: block;
}

@font-face {
    font-family: "KaTeX_Fraktur";
    src: url("${formattedUrl}KaTeX_Fraktur-Regular.woff2") format("woff2"), url("${formattedUrl}KaTeX_Fraktur-Regular.woff") format("woff"), url("${formattedUrl}KaTeX_Fraktur-Regular.ttf") format("truetype");
    font-weight: normal;
    font-style: normal;
    font-display: block;
}

@font-face {
    font-family: "KaTeX_Main";
    src: url("${formattedUrl}KaTeX_Main-Bold.woff2") format("woff2"), url("${formattedUrl}KaTeX_Main-Bold.woff") format("woff"), url("${formattedUrl}KaTeX_Main-Bold.ttf") format("truetype");
    font-weight: bold;
    font-style: normal;
    font-display: block;
}

@font-face {
    font-family: "KaTeX_Main";
    src: url("${formattedUrl}KaTeX_Main-BoldItalic.woff2") format("woff2"), url("${formattedUrl}KaTeX_Main-BoldItalic.woff") format("woff"), url("${formattedUrl}KaTeX_Main-BoldItalic.ttf") format("truetype");
    font-weight: bold;
    font-style: italic;
    font-display: block;
}

@font-face {
    font-family: "KaTeX_Main";
    src: url("${formattedUrl}KaTeX_Main-Italic.woff2") format("woff2"), url("${formattedUrl}KaTeX_Main-Italic.woff") format("woff"), url("${formattedUrl}KaTeX_Main-Italic.ttf") format("truetype");
    font-weight: normal;
    font-style: italic;
    font-display: block;
}

@font-face {
    font-family: "KaTeX_Main";
    src: url("${formattedUrl}KaTeX_Main-Regular.woff2") format("woff2"), url("${formattedUrl}KaTeX_Main-Regular.woff") format("woff"), url("${formattedUrl}KaTeX_Main-Regular.ttf") format("truetype");
    font-weight: normal;
    font-style: normal;
    font-display: block;
}

@font-face {
    font-family: "KaTeX_Math";
    src: url("${formattedUrl}KaTeX_Math-BoldItalic.woff2") format("woff2"), url("${formattedUrl}KaTeX_Math-BoldItalic.woff") format("woff"), url("${formattedUrl}KaTeX_Math-BoldItalic.ttf") format("truetype");
    font-weight: bold;
    font-style: italic;
    font-display: block;
}

@font-face {
    font-family: "KaTeX_Math";
    src: url("${formattedUrl}KaTeX_Math-Italic.woff2") format("woff2"), url("${formattedUrl}KaTeX_Math-Italic.woff") format("woff"), url("${formattedUrl}KaTeX_Math-Italic.ttf") format("truetype");
    font-weight: normal;
    font-style: italic;
    font-display: block;
}

@font-face {
    font-family: "KaTeX_SansSerif";
    src: url("${formattedUrl}KaTeX_SansSerif-Bold.woff2") format("woff2"), url("${formattedUrl}KaTeX_SansSerif-Bold.woff") format("woff"), url("${formattedUrl}KaTeX_SansSerif-Bold.ttf") format("truetype");
    font-weight: bold;
    font-style: normal;
    font-display: block;
}

@font-face {
    font-family: "KaTeX_SansSerif";
    src: url("${formattedUrl}KaTeX_SansSerif-Italic.woff2") format("woff2"), url("${formattedUrl}KaTeX_SansSerif-Italic.woff") format("woff"), url("${formattedUrl}KaTeX_SansSerif-Italic.ttf") format("truetype");
    font-weight: normal;
    font-style: italic;
    font-display: block;
}

@font-face {
    font-family: "KaTeX_SansSerif";
    src: url("${formattedUrl}KaTeX_SansSerif-Regular.woff2") format("woff2"), url("${formattedUrl}KaTeX_SansSerif-Regular.woff") format("woff"), url("${formattedUrl}KaTeX_SansSerif-Regular.ttf") format("truetype");
    font-weight: normal;
    font-style: normal;
    font-display: block;
}

@font-face {
    font-family: "KaTeX_Script";
    src: url("${formattedUrl}KaTeX_Script-Regular.woff2") format("woff2"), url("${formattedUrl}KaTeX_Script-Regular.woff") format("woff"), url("${formattedUrl}KaTeX_Script-Regular.ttf") format("truetype");
    font-weight: normal;
    font-style: normal;
    font-display: block;
}

@font-face {
    font-family: "KaTeX_Size1";
    src: url("${formattedUrl}KaTeX_Size1-Regular.woff2") format("woff2"), url("${formattedUrl}KaTeX_Size1-Regular.woff") format("woff"), url("${formattedUrl}KaTeX_Size1-Regular.ttf") format("truetype");
    font-weight: normal;
    font-style: normal;
    font-display: block;
}

@font-face {
    font-family: "KaTeX_Size2";
    src: url("${formattedUrl}KaTeX_Size2-Regular.woff2") format("woff2"), url("${formattedUrl}KaTeX_Size2-Regular.woff") format("woff"), url("${formattedUrl}KaTeX_Size2-Regular.ttf") format("truetype");
    font-weight: normal;
    font-style: normal;
    font-display: block;
}

@font-face {
    font-family: "KaTeX_Size3";
    src: url("${formattedUrl}KaTeX_Size3-Regular.woff2") format("woff2"), url("${formattedUrl}KaTeX_Size3-Regular.woff") format("woff"), url("${formattedUrl}KaTeX_Size3-Regular.ttf") format("truetype");
    font-weight: normal;
    font-style: normal;
    font-display: block;
}

@font-face {
    font-family: "KaTeX_Size4";
    src: url("${formattedUrl}KaTeX_Size4-Regular.woff2") format("woff2"), url("${formattedUrl}KaTeX_Size4-Regular.woff") format("woff"), url("${formattedUrl}KaTeX_Size4-Regular.ttf") format("truetype");
    font-weight: normal;
    font-style: normal;
    font-display: block;
}

@font-face {
    font-family: "KaTeX_Typewriter";
    src: url("${formattedUrl}KaTeX_Typewriter-Regular.woff2") format("woff2"), url("${formattedUrl}KaTeX_Typewriter-Regular.woff") format("woff"), url("${formattedUrl}KaTeX_Typewriter-Regular.ttf") format("truetype");
    font-weight: normal;
    font-style: normal;
    font-display: block;
}


`}
        </style>
    )
})


KatexFontLoader.displayName = "KatexFontLoader"
