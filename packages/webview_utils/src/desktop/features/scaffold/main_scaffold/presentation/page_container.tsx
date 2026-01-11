import React, { HTMLProps, type ReactNode } from "react";
import { connect } from "react-redux";
import { ThemeMode } from "../state/main_scaffold_initial_state";
import { AppState } from "src/desktop/core/state/initial_state";
import { cn } from "@/utils/cn";
import { HealthProvider } from "src/desktop/features/health/state/health_provider";
import DesktopTitleBar from "./desktop_title_bar";
import { prefersDarkMode } from "src/desktop/core/ui_utils/prefers_dark_mode";

const connector = connect((state: AppState) => ({
    themeMode: state.scaffold.themeMode,
    theme: state.scaffold.theme,
}));

interface PageContainerProps extends HTMLProps<HTMLDivElement> {
    themeMode: AppState["scaffold"]["themeMode"];
    theme: AppState["scaffold"]["theme"];
}

const PageContainer = connector(
    ({ theme, themeMode, ...props }: PageContainerProps): ReactNode => {
        return (
            <HealthProvider>
                <div
                    {...props}
                    data-fluster-theme={theme}
                    className={cn(
                        "h-full w-full flex flex-row justify-center items-center relative bg-background text-foreground no-scrollbar-all overflow-hidden",
                        (themeMode === ThemeMode.dark ||
                            (themeMode === ThemeMode.system && prefersDarkMode())) &&
                        "dark",
                        props.className
                    )}
                >
                    <DesktopTitleBar />
                    {props.children}
                </div>
            </HealthProvider>
        );
    }
);

PageContainer.displayName = "PageContainer";

export default PageContainer;
