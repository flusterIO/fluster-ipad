import React, { useState } from "react"


/**
 * AI Generated and completely untested, but it looks decent so mess around with things as needed to stick with the gradient and shadow that that AI used.
 */
export default function NeumorphicCard() {
    const [liked, setLiked] = useState(false)
    const [likeCount, setLikeCount] = useState(248)
    const [following, setFollowing] = useState(false)

    const handleLike = () => {
        setLiked((prev) => !prev)
        setLikeCount((prev) => (liked ? prev - 1 : prev + 1))
    }

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
            {/* Inner card */}
            <div
                className="relative rounded-[calc(1.5rem-1px)] p-6 flex flex-col gap-5"
                style={{ background: "#0f1b28" }}
            >
                {/* Top glow line */}
                <div
                    className="absolute top-0 left-8 right-8 h-px"
                    style={{
                        background:
                            "linear-gradient(90deg, transparent, rgba(11,165,233,0.5), transparent)",
                    }}
                />

                {/* Header row */}
                <div className="flex items-center justify-between">
                    {/* Avatar cluster */}
                    <div className="flex items-center gap-3">
                        <div
                            className="relative w-12 h-12 rounded-2xl flex items-center justify-center text-lg font-bold"
                            style={{
                                background: "#0e1620",
                                boxShadow:
                                    "inset 3px 3px 8px #070e17, inset -3px -3px 8px #172336, 0 0 0 1.5px rgba(11,165,233,0.2)",
                                color: "#0ba5e9",
                            }}
                        >
                            AK
                        </div>
                        <div>
                            <p className="text-sm font-semibold" style={{ color: "#c8dff5" }}>
                                Alex Kim
                            </p>
                            <p className="text-xs" style={{ color: "#4d7a99" }}>
                                UI / Product Designer
                            </p>
                        </div>
                    </div>

                    {/* Follow button */}
                    <button
                        onClick={handleFollow}
                        className="relative px-4 py-1.5 rounded-xl text-xs font-semibold tracking-wide transition-all duration-200 active:scale-95"
                        style={
                            following
                                ? {
                                    background: "#0e1620",
                                    color: "#0ba5e9",
                                    boxShadow:
                                        "inset 3px 3px 7px #070e17, inset -2px -2px 6px #152030",
                                    border: "1px solid rgba(11,165,233,0.25)",
                                }
                                : {
                                    background:
                                        "linear-gradient(135deg, #0ba5e9 0%, #0980b8 100%)",
                                    color: "#0e1620",
                                    boxShadow:
                                        "4px 4px 10px #070e17, -2px -2px 8px #152030, 0 0 12px rgba(11,165,233,0.3)",
                                    border: "none",
                                }
                        }
                    >
                        {following ? "Following" : "Follow"}
                    </button>
                </div>

                {/* Divider */}
                <div
                    className="h-px w-full"
                    style={{
                        background:
                            "linear-gradient(90deg, transparent, rgba(11,165,233,0.12), transparent)",
                    }}
                />

                {/* Project preview / image area */}
                <div
                    className="relative rounded-2xl overflow-hidden h-36 flex items-center justify-center"
                    style={{
                        background: "#0e1620",
                        boxShadow:
                            "inset 4px 4px 12px #070e17, inset -3px -3px 10px #152030",
                    }}
                >
                    {/* Decorative abstract art */}
                    <svg
                        viewBox="0 0 280 130"
                        className="w-full h-full absolute inset-0"
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        <defs>
                            <radialGradient id="glow1" cx="30%" cy="50%" r="50%">
                                <stop offset="0%" stopColor="#0ba5e9" stopOpacity="0.25" />
                                <stop offset="100%" stopColor="#0ba5e9" stopOpacity="0" />
                            </radialGradient>
                            <radialGradient id="glow2" cx="75%" cy="40%" r="45%">
                                <stop offset="0%" stopColor="#0dd4aa" stopOpacity="0.18" />
                                <stop offset="100%" stopColor="#0dd4aa" stopOpacity="0" />
                            </radialGradient>
                        </defs>
                        <rect width="280" height="130" fill="url(#glow1)" />
                        <rect width="280" height="130" fill="url(#glow2)" />
                        {/* Grid lines */}
                        {[0, 40, 80, 120, 160, 200, 240, 280].map((x) => (
                            <line
                                key={`vl-${x}`}
                                x1={x}
                                y1={0}
                                x2={x}
                                y2={130}
                                stroke="rgba(11,165,233,0.06)"
                                strokeWidth="1"
                            />
                        ))}
                        {[0, 32, 64, 96, 128].map((y) => (
                            <line
                                key={`hl-${y}`}
                                x1={0}
                                y1={y}
                                x2={280}
                                y2={y}
                                stroke="rgba(11,165,233,0.06)"
                                strokeWidth="1"
                            />
                        ))}
                        {/* Wave path */}
                        <path
                            d="M0,90 C30,70 60,110 90,80 C120,50 150,100 180,70 C210,40 240,85 280,65"
                            fill="none"
                            stroke="rgba(11,165,233,0.55)"
                            strokeWidth="2"
                            strokeLinecap="round"
                        />
                        <path
                            d="M0,90 C30,70 60,110 90,80 C120,50 150,100 180,70 C210,40 240,85 280,65 L280,130 L0,130 Z"
                            fill="rgba(11,165,233,0.07)"
                        />
                        {/* Accent dots */}
                        <circle cx="90" cy="80" r="3.5" fill="#0ba5e9" opacity="0.9" />
                        <circle cx="180" cy="70" r="3.5" fill="#0dd4aa" opacity="0.9" />
                        <circle cx="240" cy="85" r="2.5" fill="#0ba5e9" opacity="0.6" />
                        {/* Label */}
                        <text
                            x="14"
                            y="22"
                            fill="rgba(11,165,233,0.7)"
                            fontSize="9"
                            fontFamily="monospace"
                            letterSpacing="1.5"
                        >
                            PORTFOLIO · 2025
                        </text>
                    </svg>
                </div>

                {/* Card body text */}
                <div className="flex flex-col gap-1.5">
                    <h2
                        className="text-base font-semibold leading-snug text-balance"
                        style={{ color: "#c8dff5" }}
                    >
                        Stellar Dashboard Concept
                    </h2>
                    <p
                        className="text-xs leading-relaxed"
                        style={{ color: "#4d7a99" }}
                    >
                        A dark-mode analytics interface exploring depth through neumorphic
                        surfaces and glowing accent layers.
                    </p>
                </div>

                {/* Tags */}
                <div className="flex items-center gap-2 flex-wrap">
                    {["UI Design", "Neumorphism", "Data Viz"].map((tag) => (
                        <span
                            key={tag}
                            className="px-3 py-1 rounded-xl text-[10px] font-medium tracking-wide"
                            style={{
                                background: "#0e1620",
                                color: "#0ba5e9",
                                boxShadow:
                                    "inset 2px 2px 5px #070e17, inset -1px -1px 4px #152030",
                                border: "1px solid rgba(11,165,233,0.1)",
                            }}
                        >
                            {tag}
                        </span>
                    ))}
                </div>

                {/* Divider */}
                <div
                    className="h-px w-full"
                    style={{
                        background:
                            "linear-gradient(90deg, transparent, rgba(11,165,233,0.12), transparent)",
                    }}
                />

                {/* Footer stats + actions */}
                <div className="flex items-center justify-between">
                    {/* Stats */}
                    <div className="flex items-center gap-4">
                        <NeuStat label="Views" value="12.4k" />
                        <NeuStat label="Saves" value="1.8k" />
                    </div>

                    {/* Action buttons */}
                    <div className="flex items-center gap-2">
                        {/* Like */}
                        <NeuIconButton
                            active={liked}
                            onClick={handleLike}
                            label={String(likeCount)}
                            activeColor="#0ba5e9"
                        >
                            <HeartIcon filled={liked} />
                        </NeuIconButton>

                        {/* Share */}
                        <NeuIconButton active={false} label="Share">
                            <ShareIcon />
                        </NeuIconButton>
                    </div>
                </div>
            </div>
        </div>
    )
}

/* ─── Sub-components ──────────────────────────────────── */

function NeuStat({ label, value }: { label: string; value: string }) {
    return (
        <div className="flex flex-col gap-0.5">
            <span className="text-xs font-semibold" style={{ color: "#c8dff5" }}>
                {value}
            </span>
            <span className="text-[10px]" style={{ color: "#4d7a99" }}>
                {label}
            </span>
        </div>
    )
}

function NeuIconButton({
    active,
    onClick,
    label,
    activeColor = "#0ba5e9",
    children,
}: {
    active: boolean
    onClick?: () => void
    label?: string
    activeColor?: string
    children: React.ReactNode
}) {
    return (
        <button
            onClick={onClick}
            className="flex items-center gap-1.5 px-3 py-1.5 rounded-xl transition-all duration-150 active:scale-95"
            style={
                active
                    ? {
                        background: "#0e1620",
                        boxShadow:
                            "inset 3px 3px 7px #070e17, inset -2px -2px 6px #152030",
                        color: activeColor,
                    }
                    : {
                        background: "#0e1620",
                        boxShadow:
                            "3px 3px 8px #070e17, -2px -2px 7px #152030",
                        color: "#4d7a99",
                    }
            }
        >
            {children}
            {label && (
                <span className="text-[10px] font-medium" style={{ color: "inherit" }}>
                    {label}
                </span>
            )}
        </button>
    )
}

function HeartIcon({ filled }: { filled: boolean }) {
    return (
        <svg
            width="14"
            height="14"
            viewBox="0 0 24 24"
            fill={filled ? "currentColor" : "none"}
            stroke="currentColor"
            strokeWidth="2"
            strokeLinecap="round"
            strokeLinejoin="round"
        >
            <path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z" />
        </svg>
    )
}

function ShareIcon() {
    return (
        <svg
            width="14"
            height="14"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            strokeLinecap="round"
            strokeLinejoin="round"
        >
            <circle cx="18" cy="5" r="3" />
            <circle cx="6" cy="12" r="3" />
            <circle cx="18" cy="19" r="3" />
            <line x1="8.59" y1="13.51" x2="15.42" y2="17.49" />
            <line x1="15.41" y1="6.51" x2="8.59" y2="10.49" />
        </svg>
    )
}
