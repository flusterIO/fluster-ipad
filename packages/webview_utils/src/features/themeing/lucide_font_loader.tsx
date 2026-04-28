import { type GlobalWebviewStateDeepNullable } from '#/webview_global_state/cross_language_state_types';
import { withTrailingSlash } from '@/utils/string_utils';
import React, { type ReactNode } from 'react'

import { connect } from 'react-redux';
const connector = connect((state: GlobalWebviewStateDeepNullable) => ({
    fontUrl: state.math.mathjax_font_url
}))

interface LucideFontLoaderProps {
    fontUrl: GlobalWebviewStateDeepNullable["math"]["mathjax_font_url"]
}

export const LucideFontLoader = connector((props: LucideFontLoaderProps): ReactNode => {
    return <style>
        {`
@font-face {
font-family: 'FontLucide';
 src: url("${withTrailingSlash(props.fontUrl)}lucide.ttf") format("truetype");
   font-weight: normal;
   font-style: normal;
   font-display: block;
}
`}
    </style>
})


LucideFontLoader.displayName = "LucideFontLoader"
