import React, { type HTMLProps, type ReactNode } from 'react'



interface AiGeneratedCardTagProps extends Omit<HTMLProps<HTMLSpanElement>, "key" | "className" | "style"> {
    body: string
}

export const AiGeneratedCardTag = ({ body, ...props }: AiGeneratedCardTagProps): ReactNode => {
    return (
        <span
            key={body}
            className="px-3 py-1 rounded-xl text-[10px] font-medium tracking-wide"
            style={{
                background: "#0e1620",
                color: "#0ba5e9",
                boxShadow:
                    "inset 2px 2px 5px #070e17, inset -1px -1px 4px #152030",
                border: "1px solid rgba(11,165,233,0.1)",
            }}
            {...props}
        >
            {body}
        </span>
    )
}


AiGeneratedCardTag.displayName = "AiGeneratedCardTag"
