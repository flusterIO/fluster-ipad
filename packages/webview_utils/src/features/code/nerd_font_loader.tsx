import { type GlobalWebviewStateDeepNullable } from '#/webview_global_state/cross_language_state_types';
import { withTrailingSlash } from '@/utils/string_utils';
import React, { type ReactNode } from 'react'

import { connect } from 'react-redux';
const connector = connect((state: GlobalWebviewStateDeepNullable) => ({
    fontUrl: state.math.mathjax_font_url
}))

interface NerdFontLoaderProps {
    fontUrl: GlobalWebviewStateDeepNullable["math"]["mathjax_font_url"]
}

export const NerdFontLoader = connector((props: NerdFontLoaderProps): ReactNode => {
    return <style>
        {`
@font-face {
font-family: 'FontNerd';
 src: url("${withTrailingSlash(props.fontUrl)}nerd.ttf") format("truetype");
   font-weight: normal;
   font-style: normal;
   font-display: block;
}
`}
    </style>
})


NerdFontLoader.displayName = "NerdFontLoader"
