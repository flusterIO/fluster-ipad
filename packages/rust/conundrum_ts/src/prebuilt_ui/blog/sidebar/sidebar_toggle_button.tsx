"use client"
import { ChevronLeft } from 'lucide-react'
import React, { type ReactNode } from 'react'


export const SidebarToggleButton = (): ReactNode => {
    return (
        <ChevronLeft
            onClick={() => window.dispatchEvent(new CustomEvent("toggle-blog-sidebar", {
                detail: {
                    open: "toggle"
                }
            }))}
        />
    )
}


SidebarToggleButton.displayName = "SidebarToggleButton"
