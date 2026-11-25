import React, { type ReactNode } from "react";

/** Renders a JSON-LD block. `<` is escaped so content can never close the script tag. */
export const JsonLd = ({ data }: { data: object | object[] }): ReactNode => {
    return (
        <script
            type="application/ld+json"
            dangerouslySetInnerHTML={{
                __html: JSON.stringify(data).replace(/</g, "\\u003c"),
            }}
        />
    );
};

JsonLd.displayName = "JsonLd";
