"use client";
import React, { useEffect, type ReactNode } from "react";

export const LockBodyScrollHack = (): ReactNode => {
    useEffect(() => {
        window.scrollTo({
            top: 0,
            behavior: "smooth",
        });
        document.body.style.maxHeight = "100vh";
        document.body.style.overflow = "hidden";
        document.querySelector("html")!.style.maxHeight = "100vh";
        document.querySelector("html")!.style.overflow = "hidden";
    }, []);
    return null;
};

LockBodyScrollHack.displayName = "LockBodyScrollHack";
