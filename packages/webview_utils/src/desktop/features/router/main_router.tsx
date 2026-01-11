import { createBrowserRouter } from "react-router";
import { SplashScreen } from "@/shared_components/loading/splash_screen";
import DesktopScaffold from "../scaffold/main_scaffold/presentation/main_scaffold";
import { DashboardPage } from "../dashboard/presentation/temp_dashboard/temp_dashboard";
import { AppRoutes } from "../navigation/data/app_routes";

export const getDesktopRouter = () => {
    return createBrowserRouter([
        /* { */
        /*     path: AppRoutes.onboarding, */
        /*     Component: OnboardingPage, */
        /* }, */
        {
            path: AppRoutes.splash_screen,
            Component: SplashScreen,
        },
        {
            Component: DesktopScaffold,
            children: [
                { index: true, Component: DashboardPage },
                /* { */
                /*     path: AppRoutes.search, */
                /*     Component: SearchResultsPage, */
                /* }, */
            ]
        },
    ]);
};
