import React, { type ReactNode } from "react";
import { TriangleAlert } from "lucide-react";
import { cn } from "@/utils/shad_utils";

export const RouteError404 = ({ expand }: { expand?: boolean }): ReactNode => {
    return (
        <div
            className={cn(
                "w-full h-full flex flex-col justify-center items-center p-4",
                expand && "min-h-[calc(100vh-4rem)]",
            )}
        >
            <div className="w-fit max-w-3xl h-fit flex flex-col justify-center items-center">
                <div className="w-fit h-fit p-3 rounded-2xl bg-muted">
                    <TriangleAlert />
                </div>
                <div>The requested resource could not be found</div>
            </div>
        </div>
    );
};

RouteError404.displayName = "RouteError404";
