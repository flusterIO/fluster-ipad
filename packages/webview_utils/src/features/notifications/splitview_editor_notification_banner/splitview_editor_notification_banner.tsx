import React, { useEffect, useRef, type ReactNode } from 'react'
import { createPortal } from 'react-dom'
import { useSplitviewEditorNotificationContext, useSplitviewEditorNotificationDispatch } from './splitview_editor_notification_banner_provider'
import { AnimatePresence, motion } from "framer-motion";
import { XIcon } from 'lucide-react';
import { cn } from '@/utils/cn';
import { EditorNotificationBanner } from './types';



const BannerNotificationItem = ({ item }: { item: EditorNotificationBanner }): ReactNode => {
    const timer = useRef<NodeJS.Timeout | null>(null);
    const dispatch = useSplitviewEditorNotificationDispatch()

    const removeSelf = (): void => {
        dispatch({
            type: "removeEditorNotifcationBannerById",
            payload: item.id
        })
    }

    useEffect(() => {
        if (item.timeout) {
            if (timer.current) {
                clearTimeout(timer.current)
            }
            timer.current = setTimeout(removeSelf, item.timeout)
        }
    }, [])
    return (
        <motion.div
            layout
            className="min-w-fit w-full max-w-[min(540px,90vw)] px-4 py-2 bg-fd-card rounded border relative"
            initial={{
                x: "-100%",
                opacity: 0,
            }}
            animate={{
                x: 0,
                opacity: 1,
            }}
            exit={{
                x: -500,
                scale: 0,
                opacity: 0
            }}
            draggable
            drag="x"
            dragConstraints={{ left: 0, right: 0 }}
            dragElastic={0.1}
            onDragEnd={(_, info) => {
                if (info.offset.x > 100) {
                    removeSelf()
                } else if (info.offset.x < -100) {
                    removeSelf()
                }
            }}
        >
            <div className="font-bold text-sm text-fd-card-foreground pr-4">{item.title}</div>
            {item.body ? (
                <div className="text-fd-card-foreground/80 text-sm ">{item.body}</div>
            ) : null}
            <XIcon
                className="absolute top-2 right-2 w-3 h-3 text-fd-card-foreground/80 cursor-pointer"
                onClick={removeSelf}
            />
        </motion.div>
    )
}


export const SplitviewEditorNotificationBanner = (): ReactNode => {
    const { banners } = useSplitviewEditorNotificationContext()
    return createPortal(
        <div
            className={cn("w-fit h-fit z-999 fixed bottom-0 left-0 right-0 flex flex-col justify-center items-center gap-y-2 max-h-screen", banners.length && "p-4 overflow-x-visible overflow-y-auto")}
        >
            <AnimatePresence>
                {banners.map((b) => {
                    return (
                        <BannerNotificationItem
                            key={b.id}
                            item={b}
                        />
                    )
                })}
            </AnimatePresence>
        </div>
        , document.getElementById("root")!)
}


SplitviewEditorNotificationBanner.displayName = "SplitviewEditorNotificationBanner"
