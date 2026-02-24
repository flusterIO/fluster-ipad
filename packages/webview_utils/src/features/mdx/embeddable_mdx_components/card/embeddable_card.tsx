import React, { type ReactNode } from 'react'
import { embeddableCardProps, EmbeddableCardPropsInput } from './embeddable_card_props'
import { WithChildren } from '@/utils/types/utility_types'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/shared_components/shad/card'
import { WithInlineMdx } from '#/mdx/components/inline_mdx_content'



export const EmbeddableCard = ({ children, InlineMdxContent, ...props }: EmbeddableCardPropsInput & WithChildren & WithInlineMdx): ReactNode => {
    const { title, desc, containerClasses } = embeddableCardProps.parse(props)
    return (
        <Card className={containerClasses}>
            <CardHeader>
                <CardTitle>
                    <InlineMdxContent
                        mdx={title}
                    />
                </CardTitle>
                {desc ? (
                    <CardDescription>
                        <InlineMdxContent
                            mdx={desc}
                        />
                    </CardDescription>
                ) : null}
            </CardHeader>
            <CardContent>
                {children}
            </CardContent>
        </Card>
    )
}


EmbeddableCard.displayName = "EmbeddableCard"
