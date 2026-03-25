import { AIGeneratedContainer } from '#/ai/presentation/ai_generated_card'
import { InlineMdxContent } from '#/mdx/components/inline_mdx_content'
import { faker } from '@faker-js/faker'
import React, { type ReactNode } from 'react'

export const AIGeneratedContainerDevWrapper = (): ReactNode => {
    return (
        <AIGeneratedContainer
            title={faker.word.words({ count: { min: 1, max: 5 } })}
            subtitle={faker.word.words({ count: { min: 1, max: 5 } })}
            icon="pentagon"
        >
            <InlineMdxContent
                mdx={faker.lorem.paragraphs({ min: 1, max: 3 })}
            />
        </AIGeneratedContainer>
    )
}


AIGeneratedContainerDevWrapper.displayName = "AIGeneratedContainerDevWrapper"
