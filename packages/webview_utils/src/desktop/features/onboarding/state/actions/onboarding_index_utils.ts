import { NavigateFunction } from "react-router";
import { useOnboardingStateDispatch } from "../onboarding_context";
import { AppRoutes } from "src/desktop/features/navigation/data/app_routes";
import { onBoardingPageData } from "../../data/onboarding_data";

export const incrementOnboardingPageIndex = (
    currentIndex: number,
    dispatch: ReturnType<typeof useOnboardingStateDispatch>,
    nav: NavigateFunction
) => {
    if (currentIndex < onBoardingPageData.length - 1) {
        dispatch({
            type: "increment_onboarding_index",
            payload: null,
        });
    } else {
        nav(AppRoutes.dashboard.toString());
    }
};

export const decrementOnboardingPageIndex = (
    currentIndex: number,
    dispatch: ReturnType<typeof useOnboardingStateDispatch>
) => {
    if (currentIndex > 0) {
        dispatch({
            type: "decrement_onboarding_index",
            payload: null,
        });
    }
};
