"use client";
import React, { type ReactNode } from "react";
import HeroSection from "./hero/hero";
import { HighlightedFeaturesSection } from "./sections/feature_section";
import EverythingYouNeedSection from "./sections/everything_you_need_section";
import { MainSponsorSection } from "./sections/sponsors_section";
import { IpadSection } from "./sections/ipad_section/ipad_section";

const LandingPage = (): ReactNode => {
    return (
        <div className="w-full min-h-screen pb-8 flex flex-col justify-start items-center overflow-x-hidden">
            <HeroSection />
            <EverythingYouNeedSection />
            <HighlightedFeaturesSection />
            <IpadSection />
            <MainSponsorSection />
        </div>
    );
};

LandingPage.displayName = "LandingPage";

export default LandingPage;
