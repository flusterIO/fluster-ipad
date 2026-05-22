import React, { type ReactNode } from 'react'
import { type MarkdownLinkResultStringified } from "@/code_gen/typeshare/conundrum"
import { useSendNotificationBanner } from '#/notifications/splitview_editor_notification_banner/send_splitview_notification_banner'
import consola from 'consola'


interface AutoInsertedMarkdownLinkProps {
    data: MarkdownLinkResultStringified
}

export const AutoInsertedMarkdownLink = ({ data }: AutoInsertedMarkdownLinkProps): ReactNode => {
    const sendNotif = useSendNotificationBanner()
    const _url = data.url as unknown as typeof data.url.content;
    if (typeof _url !== "string") {
        consola.warn("You seem to be using a feature that you're a bit early for. Conundrum will support special links in an upcoming release, and some of the code is already in place.")
        return
    }
    if (_url.startsWith("#")) {
        return <a role="button" className="cursor-pointer" onClick={() => {
            const domId = _url.replace("#", "");
            const heading = document.getElementById(`h-${domId}`);
            if (heading) {
                heading.scrollIntoView({
                    behavior: "smooth"
                })
            } else {
                const em = document.getElementById(domId);
                if (em) {
                    em.scrollIntoView({
                        behavior: "smooth"
                    })
                } else {
                    sendNotif({
                        title: "Not found",
                        body: "An element with that id could not be found on the current page.",
                        timeout: 5000
                    })
                }
            }
        }}>{data.text}</a>
    }
    return (
        <a className="cursor-pointer" href={_url}>{data.text}</a>
    )
}


AutoInsertedMarkdownLink.displayName = "AutoInsertedMarkdownLink"
