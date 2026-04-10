import { type EmojiData } from '@/code_gen/typeshare/conundrum'
import React, { useState, type ReactNode } from 'react'
import { cn } from '../../../../core/utils/cn'
import { useSendNotificationBanner } from '#/notifications/splitview_editor_notification_banner/send_splitview_notification_banner'

interface EmojiDocsDemoProps {
    data: EmojiData
}

export const EmojiDocsDemo = ({ data }: EmojiDocsDemoProps): ReactNode => {
    const [hovered, setHovered] = useState(false)
    const sendBannerNotification = useSendNotificationBanner()
    /* const ref = useRef<HTMLDivElement>(null) */
    const handleCopyEmoji = async (): Promise<void> => {
        try {
            await window.navigator.clipboard.writeText(data.name);
            sendBannerNotification({
                title: "Success",
                body: `Your ${data.name} emoji name has been copied to your clipboard.`,
                timeout: 3000
            })
        } catch (err) {
            console.error("Error: ", err)

        }
    }
    return (
        <div
            onMouseEnter={() => { setHovered(true); }}
            onMouseLeave={() => { setHovered(false); }}
            onClick={() => {
                handleCopyEmoji().catch((e: unknown) => {
                    console.error("Error: ", e);
                })
            }}
            className="w-full h-full rounded-lg border grid grid-cols-1 grid-rows-[1fr_auto] place-items-center gap-0 p-0 bg-background text-foreground cursor-pointer">
            <div className="flex-grow-1 pt-4 px-4 w-16 h-16 flex flex-col justify-center items-center" dangerouslySetInnerHTML={{ __html: data.svg }} />
            <div className={cn("text-center w-full max-w-full overflow-x-auto overflow-y-hidden px-4 py-2 transition-colors duration-300 text-sm", hovered ? "text-foreground" : "text-foreground/80")}>{data.name}</div>
        </div>
    )
}


EmojiDocsDemo.displayName = "EmojiDocsDemo"
