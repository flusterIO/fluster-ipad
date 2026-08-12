import React, { type ReactNode } from "react";
import { useGenericRemoteDataContext } from "./generic_data_loading_context";
import { LoadingIndicator } from "#/navigation/loading_indicator";

interface GenericDataLoadingSwitchProps {
    children: ReactNode;
    loadingElement?: ReactNode;
}

export const GenericDataLoadingSwitch = ({
    children,
    loadingElement,
}: GenericDataLoadingSwitchProps): ReactNode => {
    const { loading } = useGenericRemoteDataContext();
    if (loading) {
        if (loadingElement) {
            return loadingElement;
        }
        return (
            <div className="w-full h-full min-h-fit flex flex-col justify-center items-center p-3">
                <LoadingIndicator />
            </div>
        );
    }
    return children;
};

GenericDataLoadingSwitch.displayName = "GenericDataLoadingSwitch";
