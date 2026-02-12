import { BugIcon } from 'lucide-react'
import React, { type ReactNode } from 'react'
import { FallbackProps } from 'react-error-boundary'
import { ZodError } from 'zod'


const MdxErrorDisplay = ({ e }: { e: ZodError<any> | Error }): ReactNode => {
    if (!e) {
        return null
    }
    if (e instanceof ZodError) {
        return (
            <div className="grid grid-cols-[auto_1fr]">
                <div className="text-danger w-fit">Error</div>
                <div className="w-full">{e.issues.map((i) => {
                    return (
                        <div>{i.message}</div>
                    )
                })}</div>
            </div>
        )
    } else {
        return null
    }
}


export const MdxParsingErrorComponent = (props: FallbackProps): ReactNode => {
    const e = props?.error as Error | ZodError<any> | [Error] | [ZodError<any>] | null | undefined
    return (
        <div className="w-fit h-fit rounded border p-4 flex flex-col justify-center items-center">
            <div className="bg-danger text-danger-foreground rounded-[100%] p-4">
                <BugIcon />
            </div>
            {e ? (
                Array.isArray(e) ? (
                    e.map((_e) => <MdxErrorDisplay e={_e} />)
                ) : (
                    <MdxErrorDisplay e={e} />
                )
            ) : (
                <div>An mdx parsing error occurred. I'm working on a better report here.</div>
            )}
        </div>
    )
}


MdxParsingErrorComponent.displayName = "MdxParsingErrorCompontent"
