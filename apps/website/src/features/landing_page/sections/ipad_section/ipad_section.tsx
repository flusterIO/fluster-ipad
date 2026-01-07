import React, { type ReactNode } from "react";
import { IPadCarousel } from "./ipad_screenshot_carousel/carousel";

export const IpadSection = (): ReactNode => {
    return (
        <section id="features" className="py-20 px-4 w-full">
            <div className="mx-auto w-full max-w-[min(1080px,90vw)] w-full  grid grid-cols-1 xl:grid-cols-2">
                <div className="mb-16 text-center xl:text-left">
                    <h2 className="text-3xl md:text-4xl font-bold text-gray-100 mb-4">
                        Mobile Coming Soon
                    </h2>
                    <p className="text-lg text-gray-300 max-w-2xl mx-auto">
                        The iPad application is 90% complete, and should be available as
                        soon as I get a job so I can afford an Apple developer license.
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
