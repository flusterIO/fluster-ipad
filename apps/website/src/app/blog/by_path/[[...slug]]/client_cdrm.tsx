"use client";
import React, { useEffect, type ReactNode } from "react";
import { type AnyBuilderOutput } from "../../../../../../../packages/rust/conundrum_ts/dist/src/types/general";
import { initializeConundrumWeb } from "@conundrum/ts";

interface ClientConundrumPageProps {
    item: AnyBuilderOutput["files"][number];
}

export const ClientConundrumPage = ({
    item,
}: ClientConundrumPageProps): ReactNode => {
    useEffect(() => {
        initializeConundrumWeb();
    }, []);
    return (
        <div className="mx-auto max-w-[min(1080px,90vw)]">
            <div dangerouslySetInnerHTML={{ __html: item.html }} />
        </div>
    );
};

ClientConundrumPage.displayName = "ClientConundrumPage";
