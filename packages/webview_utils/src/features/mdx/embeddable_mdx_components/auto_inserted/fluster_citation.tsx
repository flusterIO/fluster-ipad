import React, { type ReactNode } from "react";

interface FlusterCitationProps {
    citationKey: string;
    /** The index that this citation appears on the page. */
    index: number;
}

export const FlusterCitation = (props: FlusterCitationProps): ReactNode => {
    // TODO: Fix this by adding a 'scroll to' on click method that scrolls to the proper citation in the document.
    return <sup className="text-primary">{props.index + 1}</sup>;
};

FlusterCitation.displayName = "FlusterCitation";
