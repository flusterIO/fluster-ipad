"use client";
import React, { useEffect, type ReactNode } from "react";
import initConundrumWeb, { search_conundrum_emojis } from "@conundrum/wasm";

export const CdrmClientLoader = (): ReactNode => {
    useEffect(() => {
        initConundrumWeb()
            .then(() => {
                if (!window.conundrum) {
                    window.conundrum = {};
                }
                window.conundrum.searchEmojis = search_conundrum_emojis;
            })
            .catch((err: unknown) => {
                console.error("Error initializing conundrum web: ", err);
            });
        /* initializeConundrumWeb(); */
    }, []);
    return null;
};

CdrmClientLoader.displayName = "CdrmClientLoader";
