import { sendSplitviewNotificationBanner } from '#/notifications/splitview_editor_notification_banner/send_splitview_notification_banner'
import { cn } from '@/utils/cn'
import { copyStringToClipboard } from '@/utils/string_utils'
import { CopyIcon } from 'lucide-react'
import React, { HTMLProps, useRef, useState, type ReactNode } from 'react'
import { BundledTheme } from 'shiki'



export const AutoInsertedCodeBlock = ({ className, children, ...props }: HTMLProps<HTMLPreElement>): ReactNode => {
    const [hovered, setHovered] = useState(false)
    const ref = useRef<HTMLPreElement | null>(null)
    const getLanguage = (): BundledTheme | undefined => {
        return ref.current?.getAttribute("data-language") as BundledTheme | undefined
    }
    return (
        <>
            <pre
                className={cn("rounded", className)}
                {...props}
                onClick={() => setHovered(!hovered)}
                ref={ref}
            >
                {children}
            </pre>
            <div
                role="button"
                className={cn("auto-codeblock-icon absolute top-2 right-2 transition-opacity duration-300", hovered && "opacity-1000 hovered iflex-col justify-center items-center")}
                onClick={async (e) => {
                    e.stopPropagation()
                    e.preventDefault()
                    const content = ref.current?.textContent
                    if (content) {
                        const success = await copyStringToClipboard(content)
                        if (success) {
                            const lang = getLanguage()
                            sendSplitviewNotificationBanner({
                                title: "Success",
                                body: lang ? `Your ${lang} code has been copied to your clipboard` : `This code has been copied to your clipboard.`,
                                timeout: 3000
                            })
                        }
                    }
                }}
            >
                <CopyIcon
                    className={cn("w-5 h-5 p-1", hovered && "opacity-100")}
                />
            </div>
        </>
    )
}


AutoInsertedCodeBlock.displayName = "AutoInsertedCodeBlock"
