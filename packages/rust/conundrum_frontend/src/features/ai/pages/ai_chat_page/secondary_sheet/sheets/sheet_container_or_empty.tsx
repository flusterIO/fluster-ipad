/* eslint-disable @typescript-eslint/no-non-null-assertion */
import React, { type FC, type ReactNode } from "react";

interface SheetContainerOrEmptyProps<T> {
    data?: T | null;
    Content: FC<{ data: T }>;
    Empty: FC;
    className?: string;
}

export const SheetContainerOrEmpty = <T extends unknown>({
    data,
    Empty,
    Content,
    className,
}: SheetContainerOrEmptyProps<T>): ReactNode => {
    if (Array.isArray(data) ? Boolean(data.length) : Boolean(data)) {
        return (
            <div className={className}>
                <Content data={data!} />
            </div>
        );
    } else {
        return (
            <div className="w-full h-fit grow flex flex-col justify-center items-center p-4">
                <Empty />
            </div>
        );
    }
};

SheetContainerOrEmpty.displayName = "SheetContainerOrEmpty";
