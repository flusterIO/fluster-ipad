"use client";
import { useEffect, type ReactNode } from "react";
import { initializeConundrumWeb } from "./initialize_conundrum_web";

export const ConundrumInitializer = (): ReactNode => {
    useEffect(() => {
        initializeConundrumWeb();
    }, []);
    return null;
};

ConundrumInitializer.displayName = "ConundrumInitializer";
