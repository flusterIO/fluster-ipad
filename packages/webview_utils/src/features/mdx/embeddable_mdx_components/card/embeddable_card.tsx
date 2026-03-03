import React, { type ReactNode } from 'react'
import { embeddableCardPropsSchema, EmbeddableCardPropsInput } from './embeddable_card_props'
import { WithChildren } from '@/utils/types/utility_types'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/shared_components/shad/card'
import { WithInlineMdx } from '#/mdx/components/inline_mdx_content'



export const EmbeddableCard = ({ children, InlineMdxContent, ...props }: EmbeddableCardPropsInput & WithChildren & WithInlineMdx): ReactNode => {
    const { title, desc, containerClasses, shrink, centerContent, centerBody } = embeddableCardPropsSchema.parse(props)
    return (
        <Card className={containerClasses} size={shrink ? "sm" : "default"}>
            <CardHeader className="w-full">
                <CardTitle>
                    <InlineMdxContent
                        mdx={title}
                        className={centerContent && "text-center [&>p]:text-center"}
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
            <CardContent
                className={(centerContent || centerBody) ? "flex flex-col justify-center items-center" : undefined}
            >
                {children}
            </CardContent>
        </Card>
    )
}


EmbeddableCard.displayName = "EmbeddableCard"
