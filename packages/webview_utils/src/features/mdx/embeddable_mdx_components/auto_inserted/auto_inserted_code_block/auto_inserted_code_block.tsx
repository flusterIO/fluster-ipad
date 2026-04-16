import { useSendNotificationBanner } from '#/notifications/splitview_editor_notification_banner/send_splitview_notification_banner'
import { cn } from '@/utils/cn'
import { copyStringToClipboard } from '@/utils/string_utils'
import consola from 'consola'
import { CopyIcon } from 'lucide-react'
import React, { type HTMLProps, useRef, useState, type ReactNode } from 'react'
import { type BundledLanguage } from 'shiki'


export const AutoInsertedCodeBlock = ({ className, children, ...props }: HTMLProps<HTMLDivElement>): ReactNode => {
    const [hovered, setHovered] = useState(false)
    const ref = useRef<HTMLDivElement | null>(null)
    const sendSplitviewNotificationBanner = useSendNotificationBanner()
    const getLanguage = (): BundledLanguage | undefined => {
        return ref.current?.getAttribute("data-language") as BundledLanguage | undefined
    }
    return (
        <>
            <div
                className={cn("rounded", className)}
                {...props}
                onClick={() => { setHovered(!hovered); }}
                ref={ref}
            >
                {children}
            </div>
            <div
                role="button"
                className={cn("auto-codeblock-icon absolute top-2 right-2 transition-opacity duration-300", hovered && "opacity-100 hovered flex-col justify-center items-center")}
                /* eslint-disable-next-line  -- Not creating another function just to make eslint happy. */
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
                        } else {
                            consola.error("Something went wrong copying this code.")
                        }
                    } else {
                        consola.error("No text-content could be found to be copied.")
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
