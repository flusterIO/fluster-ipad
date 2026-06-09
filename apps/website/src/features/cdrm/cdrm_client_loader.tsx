"use client";
import React, { useEffect, type ReactNode } from "react";
import initConundrumWeb, { search_conundrum_emojis } from "@conundrum/wasm";
import { initializeConundrumWeb } from "@conundrum/ts";

export const CdrmClientLoader = (): ReactNode => {
    useEffect(() => {
        initializeConundrumWeb();
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
    }, []);
    return null;
};

CdrmClientLoader.displayName = "CdrmClientLoader";
