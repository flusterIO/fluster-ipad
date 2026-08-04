import React, { type ReactNode } from "react";

interface TaggablesListWrapperProps {
    children: ReactNode;
}

export const TaggablesListWrapper = (
    props: TaggablesListWrapperProps,
): ReactNode => {
    return (
        <div className="flex flex-col justify-center items-center w-full max-h-62.5 overflow-x-hidden overflow-y-auto no-scrollbar pt-2">
            {props.children}
        </div>
    );
};

TaggablesListWrapper.displayName = "TaggablesListWrapper";
