"use client";
import { type RefObject, useRef, type ReactNode } from "react";
import { BackgroundGradientCard as BackgroundGradientCardComponent } from "../sponsors_section/background_gradient_card";
import { motion } from "framer-motion";

const BackgroundGradientCard = motion.create(BackgroundGradientCardComponent);

export const AlphaSection = ({
    children: compiledMath,
}: {
    children: ReactNode;
}): ReactNode => {
    const ref = useRef<HTMLDivElement>(null);
    return (
        <div className="w-screen min-h-screen relative">
            <div className="w-full max-w-[min(90vw,1240px)] px-8 md:px-12 h-fit min-h-screen flex flex-col justify-center lg:grid lg:grid-cols-2 mx-auto gap-x-6 relative">
                <motion.div className="h-full text-center lg:text-left min-h-fit flex flex-col justify-center items-start z-10">
                    <motion.div
                        className="text-4xl font-bold w-full"
                        initial={{
                            scale: 0,
                            origin: "center",
                        }}
                        whileInView={{
                            scale: 1,
                        }}
                    >
                        I <span className="italic">finally</span> have it...
                    </motion.div>
                    <motion.div
                        className="text-foreground/80 w-full"
                        initial={{
                            scaleX: 0,
                            originX: "left",
                        }}
                        whileInView={{
                            scaleX: 1,
                        }}
                        transition={{
                            delay: 0.05,
                        }}
                    >
                        It took me almost 5 years, but I found what I was looking for.
                    </motion.div>
                    <motion.div
                        className="mt-4 w-full max-w-full z-10"
                        initial={{
                            opacity: 0,
                            scale: 0,
                            origin: "center",
                        }}
                        whileInView={{
                            opacity: 1,
                            scale: 1,
                        }}
                        transition={{
                            delay: 0.1,
                        }}
                    >
                        I think I finally made the discovery that I set out after when I
                        originally created Fluster. Here, α is the 'most mysterious number
                        in the Universe', the fine structure constant that describes the
                        geometry of electromagnetism defined in terms of gravitational
                        parameters to within{" "}
                        <span className="text-white font-semibold">0.0034%</span>, and I can
                        explain <span className="italic">why</span>.
                    </motion.div>
                    <motion.div
                        className="w-full mt-6 text-foreground text-lg font-semibold"
                        initial={{
                            x: -200,
                            opacity: 0,
                        }}
                        whileInView={{
                            x: 0,
                            opacity: 1,
                        }}
                        transition={{
                            delay: 0.15,
                        }}
                    >
                        The world's about to change for the{" "}
                        <motion.span className="font-semibold italic relative flex flex-col gapy-0 w-fit inline-block">
                            better...
                            <motion.div
                                className="w-full bottom-0 bg-primary rounded-full h-[6px]"
                                initial={{
                                    scaleX: 0,
                                    originX: "left",
                                }}
                                whileInView={{
                                    scaleX: 1,
                                }}
                                transition={{
                                    delay: 1,
                                }}
                            />
                        </motion.span>
                    </motion.div>
                </motion.div>
            </div>
            <div className="w-screen bg-linear-to-b from-background via-fd-card to-background overflow-visible min-h-[50vh] md:min-h-auto lg:absolute z-10 h-full z-[1] top-0 bottom-0 right-0 left-0 grid grid-cols-1 lg:grid-cols-[1fr_1fr] place-items-center mb-16">
                <div className="w-full hidden md:block" />
                <BackgroundGradientCard
                    className="hide-math-labels max-w-full p-4 [&>*]:text-[6vw]! lg:[&>*]:text-[3vw]! border opacity-100"
                    initial={{
                        scale: 0,
                        origin: "center",
                    }}
                    whileInView={{
                        scale: 1,
                    }}
                    border
                    animate
                    backgroundRef={ref as RefObject<HTMLDivElement>}
                    classes={{
                        container: "ml-[1/2] w-fit h-fit float-right",
                        cardContainer: "rounded-[24px]",
                        card: "bg-fd-card",
                    }}
                >
                    {compiledMath}
                </BackgroundGradientCard>
            </div>
        </div>
    );
};

AlphaSection.displayName = "AlphaSection";
