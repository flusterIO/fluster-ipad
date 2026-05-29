"use client"
import { LiquidGlassCard } from "#/features/landing_page/sections/ipad_section/liquid_glass_card";
import { SearchIcon } from "lucide-react";
import React, { type ReactNode } from "react";
import { motion } from "framer-motion"

export const MobileSidebarToggleButton = (): ReactNode => {
    return (
        <motion.button
            className={"fixed bottom-4 right-4 w-10 h-10 rounded-full flex items-center justify-center shadow-lg transition-transform z-100"}
            onClick={() => {
                console.log(`Here...`)
                window.dispatchEvent(new CustomEvent("toggle-drawer"))
            }}
        >
            <LiquidGlassCard
                glowIntensity="sm"
                shadowIntensity="sm"
                borderRadius="12px"
                blurIntensity="lg"
                draggable={false}
                className="absolute w-full h-full p-4"
            />
            <SearchIcon
                className="w-6 h-6 z-10 stroke-white/80"
            />
        </motion.button>
    );
};

MobileSidebarToggleButton.displayName = "MobileSidebarToggleButton";
