import { type MarkdownHeadingStringifiedResult } from '@/code_gen/typeshare/conundrum'
import React, { useId, useMemo, useState, type ReactNode } from 'react'
import { motion } from 'framer-motion'
import { ChevronRightIcon as CI } from 'lucide-react'
import { useSendNotificationBanner } from '#/notifications/splitview_editor_notification_banner/send_splitview_notification_banner';
import { InlineMdxContent } from '#/mdx/components/inline_mdx_content';
import { useEventListener } from '@/state/hooks/use_event_listener';

const ChevronRightIcon = motion.create(CI);



interface InlineTocProps {
    toc: MarkdownHeadingStringifiedResult[]
    initiallyExpanded: boolean
}


const PADDING_SCALAR = 0.5;

interface CloseTocGroupProps {
    tocGroupId: string
}

declare global {

    interface WindowEventMap {
        "close-toc-group": CustomEvent<CloseTocGroupProps>;
    }
}

const InlineTocItem = ({ item, nested, initiallyExpanded, tocGroupId }: { tocGroupId: string, item: MarkdownHeadingStringifiedResult, nested?: TocItemWithNested[], initiallyExpanded: boolean }): ReactNode => {
    const [expanded, setExpanded] = useState(initiallyExpanded);
    const paddingLeft = `${item.tab_depth * PADDING_SCALAR}rem`
    const showNotif = useSendNotificationBanner()
    const handleItemClick = (): void => {
        const em = document.getElementById(`h-${item.id}`);
        if (em) {
            em.scrollIntoView({
                behavior: "smooth"
            })
            window.dispatchEvent(new CustomEvent("close-toc-group", {
                detail: {
                    tocGroupId
                }
            }))
        } else {
            showNotif({
                title: "Lost & Not Found",
                body: "The heading with the id you requested could not be found in this document.",
                timeout: 5000
            })
        }
    }

    useEventListener("close-toc-group", (e) => {
        if (e.detail.tocGroupId === tocGroupId) {
            setExpanded(false)
        }
    })
    if (nested?.length) {
        return (
            <div className="w-full flex flex-col justify-center items-center gap-0">
                <div
                    style={{
                        paddingLeft
                    }}
                    className="w-full grid grid-cols-[2rem_1fr] px-2 py-1 text-fd-card-foreground/80 hover:text-fd-card-foreground transition-colors duration-300"
                >
                    <ChevronRightIcon
                        className="place-self-center h-full"
                        animate={expanded ? "expanded" : "collapsed"}
                        variants={{
                            expanded: {
                                rotateZ: `90deg`
                            },
                            collapsed: {
                                rotateZ: 0
                            }
                        }}
                        onClick={() => {
                            setExpanded(!expanded)
                        }}
                    />
                    <InlineMdxContent
                        onClick={handleItemClick}
                        mdx={item.content}
                        className="w-full z-1 opacity-80 hover:opacity-100 transition-opacity duration-300 cursor-pointer"
                    />
                </div>
                <motion.div
                    className="w-full overflow-x-hidden overflow-y-auto"
                    style={{
                        paddingLeft: `${item.tab_depth * PADDING_SCALAR + 1}rem`
                    }}
                    initial={initiallyExpanded ? "expanded" : "collapsed"}
                    variants={{
                        expanded: {
                            height: "fit-content"
                        },
                        collapsed: {
                            height: 0
                        }
                    }}
                    animate={expanded ? "expanded" : "collapsed"}
                >
                    {nested.map((n) => {
                        return (
                            <InlineTocItem tocGroupId={tocGroupId} item={n.item} nested={n.nested} key={`${n.item.id}-${n.item.depth}`} initiallyExpanded={initiallyExpanded} />
                        )
                    })}
                </motion.div>
            </div>
        )
    }
    return (
        <InlineMdxContent
            style={{
                paddingLeft
            }}
            onClick={handleItemClick}
            mdx={item.content}
            className="py-1 cursor-pointer w-full text-fd-card-foreground/80 hover:text-fd-card-foreground transition-colors duration-300 opacity-80 hover:opacity-100 transition-opacity duration-300" />
    )
}


interface TocItemWithNested { item: MarkdownHeadingStringifiedResult, nested?: TocItemWithNested[] }

const consumeChildren = (items: MarkdownHeadingStringifiedResult[]): [TocItemWithNested, MarkdownHeadingStringifiedResult[]] | null => {
    if (!items.length) {
        return null
    }
    const firstItem: TocItemWithNested = {

        item: items[0],
        nested: []
    };
    let remaining = items.slice(1, items.length);
    for (const k of remaining) {
        if (k.depth > firstItem.item.depth) {
            if (typeof firstItem.nested === "undefined") {
                firstItem.nested = []
            }
            const newRes = consumeChildren(remaining)
            if (newRes) {
                const [newK, newRemaining] = newRes;
                firstItem.nested.push(newK)
                remaining = newRemaining;
            }
        } else {
            return [firstItem, remaining]
        }
    }

    return [firstItem, remaining];
}


export const getItemsFromToc = (toc: InlineTocProps["toc"], sendNotif: ReturnType<typeof useSendNotificationBanner>) => {
    if (!toc.length) {
        return []
    }
    const items: TocItemWithNested[] = [];
    do {
        const res = consumeChildren(toc);
        if (res) {
            const [newItem, remaining] = res;
            items.push(newItem);
            toc = remaining;
        } else {
            sendNotif({
                timeout: 3000,
                title: "That's strange...",
                body: "Fluster found a table of contents component without any headings."
            })
        }
    } while (toc.length);
    return items
}

export const InlineToc = ({ toc, initiallyExpanded }: InlineTocProps): ReactNode => {
    const sendNotif = useSendNotificationBanner()
    const id = useId();
    const orderedWithChildren = useMemo(() => {
        return getItemsFromToc(toc, sendNotif)
    }, [toc])
    return (
        <div className="w-full h-fit max-w-[min(1080px,100%,90vw)] bg-fd-card rounded-border px-4 py-3 rounded-lg border">
            <div className="w-full text-lg font-bold">
                Table of Contents
            </div>
            <div className="w-full">
                {orderedWithChildren.map((item) => {
                    return (
                        <InlineTocItem tocGroupId={id} initiallyExpanded={initiallyExpanded} {...item} />
                    )
                })}
            </div>
        </div>
    )
}


InlineToc.displayName = "InlineToc"
