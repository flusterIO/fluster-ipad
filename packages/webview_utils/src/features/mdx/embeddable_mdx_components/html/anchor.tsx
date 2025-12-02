import React, { HTMLProps, type ReactNode } from "react";

interface AnchorTagProps extends HTMLProps<HTMLAnchorElement> { }

export const AnchorTag = (props: AnchorTagProps): ReactNode => {
    // TODO: Come back here and make it possible to navigate to local notes from within each note using a special syntax and a message back to swift.
    return <a {...props} />;
};

AnchorTag.displayName = "AnchorTag";
