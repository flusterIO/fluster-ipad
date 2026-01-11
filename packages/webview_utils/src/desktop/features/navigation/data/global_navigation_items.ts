import { ComponentType } from "react";
import { AppRoutes } from "./app_routes";
import { NavigationItem, NavItemPosition } from "./navigation_types";
import {
    IconHome,
    IconSettings2,
} from "@tabler/icons-react";


type IconComponent = ComponentType<{ className: string }>;

export const globalNavigationItems = (): NavigationItem[] => {
    return [
        new NavigationItem(
            "Dashboard",
            AppRoutes.dashboard,
            IconHome as IconComponent,
            NavItemPosition.top
        ),
        // new NavigationItem(
        //   "AI Chat",
        //   AppRoutes.aiMainChat,
        //   LucideBubbles as IconComponent,
        //   NavItemPosition.top
        // ),
        // new NavigationItem(
        //   "Bibliography",
        //   AppRoutes.bibliography,
        //   IconBook2 as IconComponent,
        //   NavItemPosition.top
        // ),
        // new NavigationItem(
        //   "Bookmarks",
        //   AppRoutes.bookmarks,
        //   Bookmark as IconComponent,
        //   NavItemPosition.hidden
        // ),
        // new NavigationItem(
        //   "Calendar",
        //   AppRoutes.calendar,
        //   Calendar as IconComponent,
        //   NavItemPosition.top
        // ),
        // new NavigationItem(
        //   "Task Manager",
        //   AppRoutes.taskLists,
        //   CheckIcon as IconComponent,
        //   NavItemPosition.hidden
        // ),
        // new NavigationItem(
        //   "Dictionary",
        //   AppRoutes.dictionary,
        //   IconAbc as IconComponent,
        //   NavItemPosition.top
        // ),
        // new NavigationItem(
        //   "Snippets",
        //   AppRoutes.snippets,
        //   IconCode as IconComponent,
        //   NavItemPosition.top
        // ),
        // new NavigationItem(
        //   "Equations",
        //   AppRoutes.equations,
        //   IconMath as IconComponent,
        //   NavItemPosition.top
        // ),
        // new NavigationItem(
        //   "Flash Cards",
        //   AppRoutes.flashcard,
        //   Brain as IconComponent,
        //   NavItemPosition.top
        // ),
        // new NavigationItem(
        //   "Kanban Boards",
        //   AppRoutes.kanbanBoards,
        //   IconBox as IconComponent,
        //   NavItemPosition.hidden
        // ),
        new NavigationItem(
            "Settings",
            AppRoutes.settings,
            IconSettings2 as IconComponent,
            NavItemPosition.bottom
        ),
    ];
};
