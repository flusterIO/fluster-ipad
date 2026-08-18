import { PageContainer } from '@/components/general/page_container'
import React, { type ReactNode } from 'react'
import { EmptyAgents } from './empty_agents'



export const AgentsPage = (): ReactNode => {
    return (
        <PageContainer title="Agents" center>
            <EmptyAgents />
        </PageContainer>
    )
}


AgentsPage.displayName = "AgentsPage"
