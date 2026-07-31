import React, { type ReactNode } from 'react'
import { PermanentSidebarButton, type PermanentSidebarButtonProps } from './sidebar_button'
import { HomeIcon, SettingsIcon } from "lucide-react"
import consola from 'consola'
import { AppPaths } from '../app_paths'


const buttons: PermanentSidebarButtonProps[] = [
    {
        href: AppPaths.dashboard,
        icon: HomeIcon
    }
]

export const PermanentSidebar = (): ReactNode => {
    return (
        <div className="w-16 h-screen bg-background border-r flex flex-col justify-between items-center py-6 gap-y-4">
            <div className="flex flex-col justify-start items-center gap-y-4">
                {buttons.map((b) => {
                    return (
                        <PermanentSidebarButton key={b.href ?? b.id} {...b} />
                    )
                })}
            </div>
            <PermanentSidebarButton
                icon={SettingsIcon}
                href={AppPaths.settings}
            />
        </div>
    )
}


PermanentSidebar.displayName = "PermanentSidebar"
