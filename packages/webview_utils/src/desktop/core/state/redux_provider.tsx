import React, { ReactNode } from "react";
import store from "./store";
import { Provider } from "react-redux";
import { PersistGate } from "redux-persist/integration/react";
import persistStore from "redux-persist/es/persistStore";
import { SplashScreen } from "@/shared_components/loading/splash_screen";
import { AvoidSavingInitialState } from "./avoid_saving_initial_state";

interface Props {
    children: ReactNode;
}

const ReduxProvider = ({ children }: Props) => {
    const persistor = persistStore(store);

    return (
        <Provider store={store}>
            <PersistGate
                loading={<SplashScreen message="Loading saved data..." />}
                persistor={persistor}
            >
                <>
                    <AvoidSavingInitialState />
                    {children}
                </>
            </PersistGate>
        </Provider>
    );
};

ReduxProvider.displayName = "ReduxProvider";

export default ReduxProvider;
