"use client"
import React, { useEffect, useRef, useState, type ReactNode } from "react";
import { animate, motion, useMotionValue } from "framer-motion";
import image1 from "../../../../../../public/assets/images/mockups/ipad/mockups/welcome_splitview.png";
import image2 from "../../../../../../public/assets/images/mockups/ipad/mockups/paper.png";
import image3 from "../../../../../../public/assets/images/mockups/ipad/mockups/markdown.png";
import image4 from "../../../../../../public/assets/images/mockups/ipad/mockups/note-bib.png";
import image5 from "../../../../../../public/assets/images/mockups/ipad/mockups/menu.png";
import image6 from "../../../../../../public/assets/images/mockups/ipad/mockups/note_details.png";
import image7 from "../../../../../../public/assets/images/mockups/mac/mockups/mac_editor-front.png";
import image8 from "../../../../../../public/assets/images/mockups/mac/mockups/mac_docs_example-front.png";
import image9 from "../../../../../../public/assets/images/mockups/mac/mockups/mac_dashboard-front.png";
import Image, { type StaticImageData } from "next/image";
import { LiquidGlassCard } from "../liquid_glass_card";


interface ImageData {
    image: StaticImageData;
    id: string;
    title: string;
    alt: string;
}
interface CarouselProps {
    breakpoints?: Record<number, { slidesToShow: number }>;
    images: ImageData[]
}

export const IPadCarousel = ({
    breakpoints = {
        0: { slidesToShow: 1 },
        768: { slidesToShow: 1 },
        1024: { slidesToShow: 1 },
    },
    images = [
        {
            image: image7,
            id: "img-7",
            title: "Mac Editor",
            alt: "The new, built-in MacOS code editor features snippets, code completion, syntax highlighting and more.",
        },
        {
            image: image3,
            id: "img-3",
            title: "Reading View",
            alt: "The iPad application can enter a reading view for study sessions and reviewing your notes.",
        },
        {
            image: image8,
            id: "img-8",
            title: "Builtin Documentation",
            alt: "The new high-performance parser includes documentation available for each component directly within your notes for a very Jupyter or IPython like experience.",
        },
        {
            image: image1,
            id: "img-1",
            title: "Splitview Editor",
            alt: "A full scale markdown & mdx editor in a native swift iPad application.",
        },
        {
            image: image2,
            id: "img-2",
            title: "Paper View",
            alt: "The iPad application is complete with Apple's first class support for the Apple pencil.",
        },
        {
            image: image4,
            id: "img-4",
            title: "Bibliography Manager",
            alt: "For academics, students and researchers, the application integrates with a bibliography manager to organize your notes how you organize your research.",
        },
        {
            image: image6,
            id: "img-6",
            title: "Note Details",
            alt: "This application's note details summary view allows for linking and searching notes in a variety of ways.",
        },
        {
            image: image5,
            id: "img-5",
            title: "Familiar Features",
            alt: "The iPad application integrates closely with the desktop applications to share a variety of ways to link and search your notes",
        },
        {
            image: image9,
            id: "img-9",
            title: "MacOS Dashboard",
            alt: "Both the MacOS and iPad application have their own varient of a dashboard for the user to use as the home for all of their notes.",
        },
    ]
}: CarouselProps): ReactNode => {
    const [index, setIndex] = useState(0);
    const [slidesToShow, setSlidesToShow] = useState(1);
    const containerRef = useRef<HTMLDivElement>(null);

    const x = useMotionValue(0);

    // Handle responsive breakpoints
    useEffect(() => {
        const updateSlidesToShow = () => {
            const width = window.innerWidth;
            const sortedBreakpoints = Object.keys(breakpoints)
                .map(Number)
                .sort((a, b) => b - a);

            for (const bp of sortedBreakpoints) {
                if (width >= bp) {
                    setSlidesToShow(breakpoints[bp].slidesToShow);
                    break;
                }
            }
        };

        updateSlidesToShow();
        window.addEventListener("resize", updateSlidesToShow);
        return () => { window.removeEventListener("resize", updateSlidesToShow); };
    }, [breakpoints]);

    useEffect(() => {
        if (containerRef.current) {
            const containerWidth = containerRef.current.offsetWidth || 1;
            const slideWidth = containerWidth / slidesToShow;
            const targetX = -index * slideWidth;

            animate(x, targetX, {
                type: "spring",
                stiffness: 300,
                damping: 30,
            });
        }
    }, [index, slidesToShow]);

    // Reset index if it exceeds max when resizing
    useEffect(() => {
        const maxIndex = Math.max(0, images.length - slidesToShow);
        if (index > maxIndex) {
            setIndex(maxIndex);
        }
    }, [slidesToShow, index]);

    const maxIndex = Math.max(0, images.length - slidesToShow);

    return (
        <div className="w-full lg:p-10 sm:p-4 p-2 flex flex-col justify-center items-center">
            <div className="flex flex-col gap-3 w-[min(640px,90vw)]">
                <div className="relative overflow-hidden rounded-lg" ref={containerRef}>
                    <motion.div className="flex flex-row" style={{ x }}>
                        {images.map((item) => (
                            <div
                                key={item.id}
                                className="relative shrink-0 h-[min(768px,80vh)] w-[min(640px,90vw)] rounded-lg overflow-hidden relative flex flex-col justify-center items-center"
                            >
                                <Image
                                    src={item.image}
                                    alt={item.alt}
                                    className="w-fit h-full z-10 h-full select-none pointer-events-none object-contain"
                                    draggable={false}
                                />
                            </div>
                        ))}
                    </motion.div>

                    {/* Navigation Buttons */}
                    <motion.button
                        disabled={index === 0}
                        onClick={() => { setIndex((i) => Math.max(0, i - 1)); }}
                        className={`absolute left-4 top-1/2 -translate-y-1/2 w-10 h-10 rounded-full flex items-center justify-center shadow-lg transition-transform z-10
              ${index === 0
                                ? "bg-muted bg-gray-300"
                                : "bg-muted hover:scale-110 hover:opacity-100 opacity-70"
                            }`}
                    >
                        <LiquidGlassCard
                            glowIntensity="sm"
                            shadowIntensity="sm"
                            borderRadius="12px"
                            blurIntensity="lg"
                            draggable={false}
                            className="absolute w-full h-full p-4"
                        />
                        <svg
                            className="w-6 h-6 z-10 stroke-white/80"
                            fill="none"
                            viewBox="0 0 24 24"
                        >
                            <path
                                strokeLinecap="round"
                                strokeLinejoin="round"
                                strokeWidth={2}
                                d="M15 19l-7-7 7-7"
                            />
                        </svg>
                    </motion.button>

                    <motion.button
                        disabled={index === maxIndex}
                        onClick={() => { setIndex((i) => Math.min(maxIndex, i + 1)); }}
                        className={`absolute right-4 top-1/2 -translate-y-1/2 w-10 h-10 rounded-full flex items-center justify-center shadow-lg transition-transform z-10
              ${index === maxIndex
                                ? "bg-muted bg-gray-300"
                                : "bg-muted hover:scale-110 hover:opacity-100 opacity-70"
                            }`}
                    >
                        <LiquidGlassCard
                            glowIntensity="sm"
                            shadowIntensity="sm"
                            borderRadius="12px"
                            blurIntensity="lg"
                            draggable={false}
                            className="absolute w-full h-full p-4"
                        />
                        <svg
                            className="w-6 h-6 z-10 stroke-white/80"
                            fill="none"
                            viewBox="0 0 24 24"
                        >
                            <path
                                strokeLinecap="round"
                                strokeLinejoin="round"
                                strokeWidth={2}
                                d="M9 5l7 7-7 7"
                            />
                        </svg>
                    </motion.button>
                    <div className="absolute bottom-4 left-1/2 -translate-x-1/2 flex gap-2 z-10">
                        {images.map((_, i) => (
                            <LiquidGlassCard
                                glowIntensity={i === index ? "lg" : "sm"}
                                shadowIntensity="sm"
                                borderRadius="12px"
                                blurIntensity="lg"
                                draggable={false}
                                key={`pos-indc-${i}`}
                                onClick={() => { setIndex(i); }}
                                className={`h-2 rounded-full transition-all bg-muted  cursor-pointer ${i === index ? "w-8" : "w-2"}`}
                            />
                        ))}
                    </div>
                </div>
            </div>
        </div>
    );
};

IPadCarousel.displayName = "IpadCarousel";
