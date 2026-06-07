import { useSendNotificationBanner } from '#/notifications/splitview_editor_notification_banner/send_splitview_notification_banner';
import { ConundrumWebEvents, CopyToClipboardSource, type SupportedCodeBlockSyntax } from '@/code_gen/typeshare/conundrum'
import { MdxPreviewWebviewActions } from '@/code_gen/typeshare/fluster_core_utilities';
import { useEventListener } from '@/state/hooks/use_event_listener'
import { sendToSwift } from '@/utils/bridge/send_to_swift';
import { type ReactNode } from 'react'

declare global {
    interface WindowEventMap {
        [ConundrumWebEvents.CodeblockCopied]: CustomEvent<{ lang?: SupportedCodeBlockSyntax }>;
    }
}

export const ConundrumListener = (): ReactNode => {
    const showNotif = useSendNotificationBanner();
    useEventListener(ConundrumWebEvents.CodeblockCopied, () => {
        showNotif({
            title: "Success",
            body: `Your code has been copied to your clipboard`,
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

        useEventListener(ConundrumWebEvents.DictionaryEntryLabelClick as keyof WindowEventMap, (e) => {
            const target = e.currentTarget as HTMLDivElement;
            const noteId = target.getAttribute("data-cdrm-noteid");
            if (noteId) {
                sendToSwift(MdxPreviewWebviewActions.ViewNoteById, noteId)
            }
        })

        const notif = notificationMap[e.detail.source as keyof typeof notificationMap];
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


ConundrumListener.displayName = "ConundrumListener"
