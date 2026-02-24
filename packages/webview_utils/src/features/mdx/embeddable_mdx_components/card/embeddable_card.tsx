import React, { type ReactNode } from 'react'
import { EmbeddableCardProps, EmbeddableCardPropsInput } from './embeddable_card_props'



export const EmbeddableCard = (props: EmbeddableCardPropsInput): ReactNode => {
    console.log("props: ", props)
    return (
        <div></div>
    )
}


EmbeddableCard.displayName = "EmbeddableCard"
