import React, { useState, type ReactNode } from 'react'
import { motion } from "framer-motion"
import { Button } from '@/shared_components/shad/button'
import { useEventListener } from '@/state/hooks/use_event_listener';
import { SummaryStreamContainerBackdrop } from './summary_stream_container_backdrop';
import { ByteBuffer } from 'flatbuffers';
import { SerializedString } from '@/code_gen/flat_buffer/shared-webview-data';
declare global {

    interface Window {
        "sendNoteSummaryStream": (content: Uint8Array, isStreamStart: boolean) => void;
    }
    interface WindowEventMap {
        "show-summary-preview": CustomEvent<boolean>;
    }
}


export const SummaryStreamContainer = (): ReactNode => {
    const [showing, setShowing] = useState(false)

    useEventListener("show-summary-preview", () => { setShowing(true); })

    if (!showing) {
        return null
    }

    return (
        <SummaryStreamContainerBackdrop>
            <motion.div
                className='w-[min(768px,90vw)] h-[min(500px,90vh)]'
            >
                <div>Summary</div>
                <div id="note-summary-stream-body" className="italic">
                    Generating Summary...
                </div>
                <div className="w-full h-fit flex flex-row justify-end items-center gap-x-3">
                    <Button
                        variant={"destructive"}
                    >
                        Deny
                    </Button>
                    <Button>
                        Accept
                    </Button>
                </div>
            </motion.div>
        </SummaryStreamContainerBackdrop>
    )
}

SummaryStreamContainer.displayName = "SummaryStreamContainer"

window.sendNoteSummaryStream = (s, isStreamStart) => {
    if (isStreamStart) {
        window.dispatchEvent(new CustomEvent("show-summary-preview", {
            detail: true
        }))
    }
    const em = document.getElementById("note-summary-stream-body")
    if (!em) {
        console.error("Could not find streaming summary body.")
    } else {
        const data = Uint8Array.from(s)
        const buf = new ByteBuffer(data)
        em.innerHTML = SerializedString.getRootAsSerializedString(buf).body() ?? ""
    }
}


