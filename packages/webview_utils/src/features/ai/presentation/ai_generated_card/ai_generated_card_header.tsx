import React, { type ReactNode } from 'react'
import { type IconName, DynamicIcon } from "lucide-react/dynamic"



interface AIGeneratedCardHeaderProps {
    title: string
    subtitle: string
    icon: IconName
}

export const AIGeneratedCardHeader = ({ title, subtitle, icon }: AIGeneratedCardHeaderProps): ReactNode => {
    return (
        <div className="flex items-center gap-3">
            <div
                className="relative w-12 h-12 rounded-2xl flex items-center justify-center text-lg font-bold"
                style={{
                    background: "#0e1620",
                    boxShadow:
                        "inset 3px 3px 8px #070e17, inset -3px -3px 8px #172336, 0 0 0 1.5px rgba(11,165,233,0.2)",
                    color: "#0ba5e9",
                }}
            >
                <DynamicIcon name={icon} />
            </div>
            <div>
                <p className="text-sm font-semibold" style={{ color: "#c8dff5" }}>
                    {title}
                </p>
                <p className="text-xs" style={{ color: "#4d7a99" }}>
                    {subtitle}
                </p>
            </div>
        </div>
    )
}


AIGeneratedCardHeader.displayName = "AIGeneratedCardHeader"
