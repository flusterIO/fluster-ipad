"use client";
import React, { type ReactNode } from "react";
import HeroSection from "./hero/hero";
import { HighlightedFeaturesSection } from "./sections/feature_section";
import { PaperSection } from "./sections/paper_section/paper_section";
import { IpadSection } from "./sections/ipad_section/ipad_section";
// import { AlphaSection } from "./sections/alpha/alpha_section";

/* export interface RenderedNestedLandingPageSlots { */
/*     alphaEquation: string; */
/* } */

export const LandingPageClientContent = (): ReactNode => {
    return (
        <>
            <HeroSection />
            <IpadSection />
            <HighlightedFeaturesSection />
            <PaperSection />
            {/* <MainSponsorSection /> */}
        </>
    );
};

LandingPageClientContent.displayName = "LandingPageClientContent";
