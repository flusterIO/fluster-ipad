"use client";
import React, { useEffect, type ReactNode } from "react";
import dynamic from "next/dynamic";
const CdrmClientLoader = dynamic(
    () => import("./cdrm_client_loader").then((a) => a.CdrmClientLoader),
    {
        ssr: false,
    },
);

export const ConundrumLoader = (): ReactNode => {
    useEffect(() => {
        console.log("Here though");
    }, []);
    return <CdrmClientLoader />;
};

ConundrumLoader.displayName = "ConundrumLoader";
