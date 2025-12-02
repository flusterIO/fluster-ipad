import React, { type ReactNode } from "react";

interface AutoInsertedTagProps {
    children: ReactNode;
}

export const AutoInsertedTag = ({
    children,
}: AutoInsertedTagProps): ReactNode => {
    return <a className="bg-primary px-2 rounded">{children}</a>;
};

AutoInsertedTag.displayName = "AutoInsertedTag";
