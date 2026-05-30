import { useSendNotificationBanner } from '#/notifications/splitview_editor_notification_banner/send_splitview_notification_banner';
import { ConundrumWebEvents, CopyToClipboardSource, type SupportedCodeBlockSyntax } from '@/code_gen/typeshare/conundrum'
import { useEventListener } from '@/state/hooks/use_event_listener'
import { type ReactNode } from 'react'

declare global {

    interface WindowEventMap {
        [ConundrumWebEvents.CodeblockCopied]: CustomEvent<{ lang?: SupportedCodeBlockSyntax }>;
    }
}

export const ConundrumNotificationListener = (): ReactNode => {
    const showNotif = useSendNotificationBanner();
    useEventListener(ConundrumWebEvents.CodeblockCopied, (e) => {
        showNotif({
            title: "Success",
            body: `Your ${e.detail.lang ?? ""} code has been copied to your clipboard`,
            timeout: 3000
        })
    })
    useEventListener(ConundrumWebEvents.CopyToClipboard, (e) => {
        const notificationMap: Partial<
            Record<CopyToClipboardSource, { title: (src: string) => string; body: (src: string) => string, timeout: number }>
        > = {
            [CopyToClipboardSource.EmojiName]: {
                title: () => "Success",
                body: (src) => `Your emoji name \`${src}\` has been successfully copied to your clipboard.`,
                timeout: 3000
            },
        };

        const notif = notificationMap[e.detail.source];
        if (notif) {
            showNotif({
                title: notif.title(e.detail.source),
                body: notif.body(e.detail.source),
                timeout: notif.timeout
            })
        }
    })
    return null
}


ConundrumNotificationListener.displayName = "ConundrumNotificationListener"
