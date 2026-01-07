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
                            Mobile App Coming Soon
                        </h2>
                    </div>
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
