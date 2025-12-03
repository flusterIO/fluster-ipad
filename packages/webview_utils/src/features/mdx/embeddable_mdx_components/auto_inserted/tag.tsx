import React, { type ReactNode } from "react";

interface AutoInsertedTagProps {
    children: ReactNode;
}

export const AutoInsertedTag = ({
    children,
}: AutoInsertedTagProps): ReactNode => {
    return (
        <span role="button" className="bg-primary px-8 rounded-md">
            {children}
        </span>
    );
};

AutoInsertedTag.displayName = "AutoInsertedTag";
