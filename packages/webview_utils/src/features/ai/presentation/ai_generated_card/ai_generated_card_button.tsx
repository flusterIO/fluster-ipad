import React, { type ReactNode } from 'react'



interface AIGeneratedCardButtonProps {
    onClick?: () => void
    active?: boolean
    children: ReactNode
}

export const AIGeneratedCardButton = ({ onClick, active, children }: AIGeneratedCardButtonProps): ReactNode => {
    return (
        <button
            onClick={onClick}
            className="relative px-4 py-1.5 rounded-xl text-xs font-semibold tracking-wide transition-all duration-200 active:scale-95"
            style={
                active
                    ? {
                        background: "#0e1620",
                        color: "#0ba5e9",
                        boxShadow:
                            "inset 3px 3px 7px #070e17, inset -2px -2px 6px #152030",
                        border: "1px solid rgba(11,165,233,0.25)",
                    }
                    : {
                        background:
                            "linear-gradient(135deg, #0ba5e9 0%, #0980b8 100%)",
                        color: "#0e1620",
                        boxShadow:
                            "4px 4px 10px #070e17, -2px -2px 8px #152030, 0 0 12px rgba(11,165,233,0.3)",
                        border: "none",
                    }
            }
        >
            {children}
        </button>
    )
}


AIGeneratedCardButton.displayName = "AIGeneratedCardButton"
