import { SunMoonIcon } from "lucide-react";
import React, { useMemo, useRef, useState, type ReactNode } from "react";
import { NavLink, useMatch } from "react-router";
import { NavigationItem, NavItemPosition } from "../../data/navigation_types";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/shared_components/shad/tooltip";
import { AppRoutes } from "../../data/app_routes";
import { useDesktopScaffoldContext } from "src/desktop/features/scaffold/main_scaffold/state/main_scaffold_context";
import { toggleDarkMode } from "src/desktop/features/scaffold/main_scaffold/main_scaffold_actions";

const SideNavigationItem = ({ item }: { item: NavigationItem }): ReactNode => {
    const Icon = item.icon;
    const isCurrent = useMatch(item.href);
    const [showTooltip, setShowTooltip] = useState(false);
    const timer = useRef<null | NodeJS.Timeout>(null);
    return (
        <Tooltip
            open={showTooltip}
            onOpenChange={(newOpen) => {
                if (!newOpen) {
                    setShowTooltip(false);
                }
            }}
        >
            <TooltipContent className="bg-card text-card-foreground [&_svg]:fill-card [&_svg]:bg-card">
                {item.label}
            </TooltipContent>
            <TooltipTrigger>
                <NavLink
                    to={item.href}
                    end
                    onMouseEnter={() => {
                        if (item.href === AppRoutes.dashboard) {
                            return;
                        }
                        timer.current = setTimeout(() => setShowTooltip(true), 500);
                    }}
                    onMouseLeave={() => {
                        if (timer.current) {
                            clearTimeout(timer.current);
                        }
                        setShowTooltip(false);
                    }}
                >
                    <Icon
                        className={isCurrent ? "text-foreground" : "text-foreground/70"}
                    />
                </NavLink>
            </TooltipTrigger>
        </Tooltip>
    );
};

const SideNavigationCol = ({
    items,
}: {
    items: NavigationItem[];
}): ReactNode => {
    return (
        <div className="flex flex-col justify-center items-center gap-6">
            {items.map((k) => (
                <SideNavigationItem item={k} key={`${k.label}-side-nav`} />
            ))}
        </div>
    );
};

const DesktopSideNavigation = (): ReactNode => {
    const { sideNavButtons } = useDesktopScaffoldContext();
    const { top, bottom } = useMemo(() => {
        const top: NavigationItem[] = [];
        const bottom: NavigationItem[] = [];
        for (const item of sideNavButtons) {
            if (item.position === NavItemPosition.top) {
                top.push(item);
            } else if (item.position === NavItemPosition.bottom) {
                bottom.push(item);
            }
        }
        return { top, bottom };
    }, [sideNavButtons]);
    return (
        <div className="w-fit h-full flex flex-col justify-between items-center gap-8 border-r pt-10 px-2 pb-6 stroke-foreground overflow-y-auto ">
            <SideNavigationCol items={top} />
            <div className="flex flex-col justify-center items-center gap-6">
                <SunMoonIcon className="cursor-pointer" onClick={toggleDarkMode} />
                {bottom.map((k) => (
                    <SideNavigationItem item={k} key={`${k.label}-side-nav`} />
                ))}
            </div>
        </div>
    );
};

DesktopSideNavigation.displayName = "DesktopSideNavigation";

export default DesktopSideNavigation;
