import { AiParsePendingErrorStatusContainer } from '#/ai/presentation/ai_parse_pending_error_status_container'
import React, { type ReactNode } from 'react'
import { faker } from "@faker-js/faker"

export const AiParsePendingErrorStateWrapper = (): ReactNode => {
    return (
        <AiParsePendingErrorStatusContainer
            error={{
                message: () => faker.lorem.words({ min: 10, max: 50 })
            }}
        />
    )
}


AiParsePendingErrorStateWrapper.displayName = "AiParsePendingErrorStateWrapper"
