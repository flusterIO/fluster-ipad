import {
    DictionaryWebviewIds,
    type GlobalWebviewStateDeepNullable,
} from "@fluster/webview_utils";
import React, { type ReactNode } from "react";
import { connect } from "react-redux";
import { DictionaryEntry } from "./dictionary_entry";

interface DictionaryPageProps {
    entries: GlobalWebviewStateDeepNullable["dictionary"]["entries"];
}

const connector = connect((state: GlobalWebviewStateDeepNullable) => ({
    entries: state.dictionary.entries,
}));

const DictionaryEmpty = (): ReactNode => {
    return (
        <div className="w-full max-w-[1080px] px-8 ml-auto mr-auto">
            <h1 className="mb-8 text-xl @md/cdrm:text-2xl @lg:/cdrm:text-3xl hide-desktop">
                Dictionary
            </h1>
            <div
                id={DictionaryWebviewIds.DictionaryDataContainer}
                className="w-full h-full flex flex-col justify-center items-center p-4"
            >
                <h3 className="text-foreground/80 w-fit text-center">
                    No Dictionary Entries Found
                </h3>
            </div>
        </div>
    );
};

export const DictionaryPage = connector(
    ({ entries }: DictionaryPageProps): ReactNode => {
        if (!entries.length) {
            return <DictionaryEmpty />;
        }
        return (
            <div className="w-full max-w-[1080px] mx-auto py-8 px-6">
                {entries
                    .sort((a, b) => (a.label < b.label ? -1 : 1))
                    .map((e) => {
                        return <DictionaryEntry entry={e} key={e.label} />;
                    })}
            </div>
        );
    },
);

DictionaryPage.displayName = "DictionaryPage";
