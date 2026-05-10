"use client";
import React, { type ReactNode } from "react";
import HeroSection from "./hero/hero";
import { HighlightedFeaturesSection } from "./sections/feature_section";
import { IpadSection } from "./sections/ipad_section/ipad_section";
import { AlphaSection } from "./sections/alpha/alpha_section";

/* export interface RenderedNestedLandingPageSlots { */
/*     alphaEquation: string; */
/* } */

export const LandingPageClientContent = ({
    children,
}: {
    children: ReactNode;
}): ReactNode => {
    return (
        <>
            <HeroSection />
            <AlphaSection>{children}</AlphaSection>
            <IpadSection />
            <HighlightedFeaturesSection />
            {/* <MainSponsorSection /> */}
        </>
    );
};

LandingPageClientContent.displayName = "LandingPageClientContent";
