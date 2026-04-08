import { type GlobalWebviewStateDeepNullable } from '#/webview_global_state/cross_language_state_types'
import React, { type ReactNode } from 'react'
import { useSelector } from 'react-redux'



interface EquationReferenceOutputProps {
    idx: number;
    id: string;
    super?: boolean
    sub?: boolean
}

export const EquationReferenceOutput = ({ idx, super: _super, sub: subscript }: EquationReferenceOutputProps): ReactNode => {
    /* const idx = useSelector((state: GlobalWebviewStateDeepNullable) => state.math.equation_refs[id]) */

    if (_super) {
        return (
            <sup className="text-sm">{idx}</sup>
        )
    } else if (subscript) {
        return (
            <sub className="text-sm">{idx}</sub>
        )
    } else {
        return (
            <span>{idx}</span>
        )
    }
}


EquationReferenceOutput.displayName = "EquationReferenceOutput"
