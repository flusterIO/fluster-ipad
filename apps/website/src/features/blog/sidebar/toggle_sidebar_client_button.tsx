"use client"
import React, { type ReactNode } from 'react'
import { SidebarToggleButton } from "@conundrum/ts/ui/blog";


export const ToggleSidebarClientButton = (): ReactNode => {
    return (
        <SidebarToggleButton />
    )
}


ToggleSidebarClientButton.displayName = "ToggleSidebarClientButton"
