import React, { type ReactNode } from 'react'
import { equationReferencePropsSchema, type EquationReferenceProps } from './equation_reference_props'
import { EquationReferenceOutput } from './equation_reference_output'


export const EquationReference = ({ ..._props }: EquationReferenceProps): ReactNode => {
    const { id, superscript, subscript, idx } = equationReferencePropsSchema.parse(_props)
    return (
        <EquationReferenceOutput idx={idx} super={superscript} sub={subscript} id={id} />
    )
}

EquationReference.displayName = "EquationReference"
