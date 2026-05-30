import React, { type ReactNode } from "react";
import { IPadCarousel } from "./ipad_screenshot_carousel/carousel";

export const IpadSection = (): ReactNode => {
    return (
        <section id="features" className="pt-20 px-4 w-full">
            <div className="mx-auto w-full max-w-[min(1080px,90vw)] w-full flex flex-col justify-center items-center">
                <div className="mb-8 text-center">
                    <div className="flex flex-col justify-center items-center">
                        <div className="text-primary text-4xl md:text-5xl font-extrabold">
                            New
                        </div>
                        <h2 className="text-3xl md:text-4xl font-bold text-gray-100 mb-4">
                            iPad & Mac Apps Coming June 2026
                        </h2>
                    </div>
                    <p className="text-lg text-gray-300 max-w-2xl mx-auto">
                        After a months long complete rewrite, the new iPad and Mac apps will
                        be available soon, complete with Apple's PaperKit integration, and{" "}
                        <span className="italic">most</span> of the features Fluster users
                        are familiar with available on initial launch.
                    </p>
                </div>
                <div className="w-full">
                    <IPadCarousel />
                </div>
            </div>
        </section>
    );
};

IpadSection.displayName = "IpadSection";
