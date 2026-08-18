import { AppPaths } from '#/navigation/app_paths'
import { CircularIcon } from '#/ui/shared_utility_components/circular_icon'
import { SearchCode } from 'lucide-react'
import React, { type ReactNode } from 'react'
import { Link } from 'react-router'


export const EmptyAgents = (): ReactNode => {
    return (
        <div className="w-[min(400px,90%)] h-fit rounded bg-fd-card text-fd-card-foreground border flex flex-col justify-center items-center py-2">
            <CircularIcon
                icon={SearchCode}
                destructive
            />
            <h5 className="text-lg font-semibold">No Agents Found</h5>
            <div>
                Click <Link to={AppPaths.agent} className="text-link">here</Link> to create a new agent.
            </div>
        </div>
    )
}


EmptyAgents.displayName = "EmptyAgents"
