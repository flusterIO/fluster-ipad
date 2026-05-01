import { useSendNotificationBanner } from '#/notifications/splitview_editor_notification_banner/send_splitview_notification_banner';
import { ConundrumWebEvents, type SupportedCodeBlockSyntax } from '@/code_gen/typeshare/conundrum'
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
    return null
}


ConundrumNotificationListener.displayName = "ConundrumNotificationListener"
