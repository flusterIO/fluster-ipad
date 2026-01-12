import React, { type ReactNode } from "react";
import { Outlet } from "react-router";
import DesktopTitleBar from "./title_bar";
import { ScaffoldResizableContainer } from "./scaffold_resizable_container";

export const MainDesktopScaffold = (): ReactNode => {
    return (
        <div className="w-full h-screen">
            <DesktopTitleBar />
            <ScaffoldResizableContainer>
                <Outlet />
            </ScaffoldResizableContainer>
        </div>
    );
};

MainDesktopScaffold.displayName = "MainDesktopScaffold";
