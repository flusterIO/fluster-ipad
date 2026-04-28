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
    const formattedUrl = withTrailingSlash(props.fontUrl);
    return (
        <style>{`
@font-face {
    font-display: block;
    font-family: KaTeX_AMS;
    font-style: normal;
    font-weight: 400;
    src: url('${formattedUrl}KaTeX_AMS-Regular.woff2') format("woff2")
}

@font-face {
    font-display: block;
    font-family: KaTeX_Caligraphic;
    font-style: normal;
    font-weight: 700;
    src: url('${formattedUrl}KaTeX_Caligraphic-Bold.woff2') format("woff2")
}

@font-face {
    font-display: block;
    font-family: KaTeX_Caligraphic;
    font-style: normal;
    font-weight: 400;
    src: url('${formattedUrl}KaTeX_Caligraphic-Regular.woff2') format("woff2")
}

@font-face {
    font-display: block;
    font-family: KaTeX_Fraktur;
    font-style: normal;
    font-weight: 700;
    src: url('${formattedUrl}KaTeX_Fraktur-Bold.woff2') format("woff2")
}

@font-face {
    font-display: block;
    font-family: KaTeX_Fraktur;
    font-style: normal;
    font-weight: 400;
    src: url('${formattedUrl}KaTeX_Fraktur-Regular.woff2') format("woff2")
}

@font-face {
    font-display: block;
    font-family: KaTeX_Main;
    font-style: normal;
    font-weight: 700;
    src: url('${formattedUrl}KaTeX_Main-Bold.woff2') format("woff2")
}

@font-face {
    font-display: block;
    font-family: KaTeX_Main;
    font-style: italic;
    font-weight: 700;
    src: url('${formattedUrl}KaTeX_Main-BoldItalic.woff2') format("woff2")
}

@font-face {
    font-display: block;
    font-family: KaTeX_Main;
    font-style: italic;
    font-weight: 400;
    src: url('${formattedUrl}KaTeX_Main-Italic.woff2') format("woff2")
}

@font-face {
    font-display: block;
    font-family: KaTeX_Main;
    font-style: normal;
    font-weight: 400;
    src: url('${formattedUrl}KaTeX_Main-Regular.woff2') format("woff2")
}

@font-face {
    font-display: block;
    font-family: KaTeX_Math;
    font-style: italic;
    font-weight: 700;
    src: url('${formattedUrl}KaTeX_Math-BoldItalic.woff2') format("woff2")
}

@font-face {
    font-display: block;
    font-family: KaTeX_Math;
    font-style: italic;
    font-weight: 400;
    src: url('${formattedUrl}KaTeX_Math-Italic.woff2') format("woff2")
}

@font-face {
    font-display: block;
    font-family: "KaTeX_SansSerif";
    font-style: normal;
    font-weight: 700;
    src: url('${formattedUrl}KaTeX_SansSerif-Bold.woff2') format("woff2")
}

@font-face {
    font-display: block;
    font-family: "KaTeX_SansSerif";
    font-style: italic;
    font-weight: 400;
    src: url('${formattedUrl}KaTeX_SansSerif-Italic.woff2') format("woff2")
}

@font-face {
    font-display: block;
    font-family: "KaTeX_SansSerif";
    font-style: normal;
    font-weight: 400;
    src: url('${formattedUrl}KaTeX_SansSerif-Regular.woff2') format("woff2")
}

@font-face {
    font-display: block;
    font-family: KaTeX_Script;
    font-style: normal;
    font-weight: 400;
    src: url('${formattedUrl}KaTeX_Script-Regular.woff2') format("woff2")
}

@font-face {
    font-display: block;
    font-family: KaTeX_Size1;
    font-style: normal;
    font-weight: 400;
    src: url('${formattedUrl}KaTeX_Size1-Regular.woff2') format("woff2")
}

@font-face {
    font-display: block;
    font-family: KaTeX_Size2;
    font-style: normal;
    font-weight: 400;
    src: url('${formattedUrl}KaTeX_Size2-Regular.woff2') format("woff2")
}

@font-face {
    font-display: block;
    font-family: KaTeX_Size3;
    font-style: normal;
    font-weight: 400;
    src: url('${formattedUrl}KaTeX_Size3-Regular.woff2') format("woff2")
}

@font-face {
    font-display: block;
    font-family: KaTeX_Size4;
    font-style: normal;
    font-weight: 400;
    src: url('${formattedUrl}KaTeX_Size4-Regular.woff2') format("woff2")
}

@font-face {
    font-display: block;
    font-family: KaTeX_Typewriter;
    font-style: normal;
    font-weight: 400;
    src: url('${formattedUrl}KaTeX_Typewriter-Regular.woff2') format("woff2")
}


`}
        </style>
    )
})


KatexFontLoader.displayName = "KatexFontLoader"
