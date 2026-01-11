import React, { useEffect, type ReactNode } from "react";
import { Outlet, useLocation } from "react-router";
import DesktopSideNavigation from "src/desktop/features/navigation/presentation/sidebar_navigation/sidebar_navigation";
import { useDevelopmentLogger } from "src/desktop/features/logging/data/use_development_logger";
import PageContainer from "./page_container";
import { DarkModeObserver } from "src/desktop/core/ui_utils/dark_mode_observer";
import MathjaxScript from "src/desktop/features/math/presentation/mathax_script";
import ToastNotificationList from "src/desktop/features/notifications/toasts/presentation/toast_notification_list";
import CommandPalette from "src/desktop/features/command_palette/presentation/command_palette";
import { CommandPaletteProvider } from "src/desktop/features/command_palette/state/command_palette_provider";
import ConfirmationModalContainer from "src/desktop/features/notifications/confirmation/presentation/confirmation_modal/confirmation_modal_container";

const DesktopScaffold = (): ReactNode => {
    useDevelopmentLogger();
    const location = useLocation();
    useEffect(() => {
        window.dispatchEvent(
            new CustomEvent("page-navigate", {
                detail: {
                    location: location,
                },
            })
        );
    }, [location]);
    return (
        <PageContainer>
            <MathjaxScript />
            <DarkModeObserver />
            <DesktopSideNavigation />
            <div
                id={"scroll-target"}
                className="@container/main-panel flex-grow h-full w-full pt-8 overflow-y-auto"
            >
                <Outlet />
            </div>
            <ToastNotificationList />
            <CommandPaletteProvider>
                <CommandPalette />
            </CommandPaletteProvider>
            <ConfirmationModalContainer />
        </PageContainer>
    );
};

DesktopScaffold.displayName = "DesktopScaffold";

export default DesktopScaffold;
