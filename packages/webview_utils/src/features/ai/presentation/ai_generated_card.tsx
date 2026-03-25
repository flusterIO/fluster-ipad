import React, { type ReactNode, useState } from "react"
import { AIGeneratedCardHeader } from "./ai_generated_card/ai_generated_card_header"
import { type IconName } from "lucide-react/dynamic"


interface AiGeneratedContainerProps {
    children: ReactNode
    title?: string
    subtitle?: string
    icon?: IconName
}

/**
 * AI Generated and completely untested, but it looks decent so mess around with things as needed to stick with the gradient and shadow that that AI used.
 */
export const AIGeneratedContainer = ({ children, title, subtitle, icon }: AiGeneratedContainerProps) => {
    const [following, setFollowing] = useState(false)

    const handleFollow = () => { setFollowing((prev) => !prev); }

    return (
        <div
            className="relative w-[360px] rounded-3xl p-px overflow-hidden"
            style={{
                background: "linear-gradient(135deg, rgba(11,165,233,0.18) 0%, rgba(14,22,32,0) 60%)",
                boxShadow:
                    "8px 8px 20px #070e17, -6px -6px 18px #152030, 0 0 40px rgba(11,165,233,0.07)",
            }}
        >
            <div
                className="relative rounded-[calc(1.5rem-1px)] p-6 flex flex-col gap-5"
                style={{ background: "#0f1b28" }}
            >
                <div
                    className="absolute top-0 left-8 right-8 h-px"
                    style={{
                        background:
                            "linear-gradient(90deg, transparent, rgba(11,165,233,0.5), transparent)",
                    }}
                />

                <div className="flex items-center justify-between not-prose">
                    {title && subtitle && icon ? (
                        <AIGeneratedCardHeader title={title} subtitle={subtitle} icon={icon} />
                    ) : null}

                </div>

                {/* Divider */}
                <div
                    className="h-px w-full"
                    style={{
                        background:
                            "linear-gradient(90deg, transparent, rgba(11,165,233,0.12), transparent)",
                    }}
                />

                {/* Card body text */}
                <div className="flex flex-col gap-1.5">
                    {children}
                </div>

                {/* Divider */}
                <div
                    className="h-px w-full"
                    style={{
                        background:
                            "linear-gradient(90deg, transparent, rgba(11,165,233,0.12), transparent)",
                    }}
                />

                {/* <div className="flex items-center justify-between"> */}
                {/*     <div className="flex items-center gap-4"> */}
                {/*         <NeuStat label="Views" value="12.4k" /> */}
                {/*         <NeuStat label="Saves" value="1.8k" /> */}
                {/*     </div> */}

                {/*     <div className="flex items-center gap-2"> */}
                {/*         <NeuIconButton */}
                {/*             active={liked} */}
                {/*             onClick={handleLike} */}
                {/*             label={String(likeCount)} */}
                {/*             activeColor="#0ba5e9" */}
                {/*         > */}
                {/*             <HeartIcon filled={liked} /> */}
                {/*         </NeuIconButton> */}

                {/*         <NeuIconButton active={false} label="Share"> */}
                {/*             <ShareIcon /> */}
                {/*         </NeuIconButton> */}
                {/*     </div> */}
                {/* </div> */}
            </div>
        </div>
    )
}

/* ─── Sub-components ──────────────────────────────────── */

/* function NeuStat({ label, value }: { label: string; value: string }) { */
/*     return ( */
/*         <div className="flex flex-col gap-0.5"> */
/*             <span className="text-xs font-semibold" style={{ color: "#c8dff5" }}> */
/*                 {value} */
/*             </span> */
/*             <span className="text-[10px]" style={{ color: "#4d7a99" }}> */
/*                 {label} */
/*             </span> */
/*         </div> */
/*     ) */
/* } */
