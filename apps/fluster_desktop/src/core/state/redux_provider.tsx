import React, { ReactNode } from "react";
import store from "./store";
import { Provider } from "react-redux";
import { PersistGate } from "redux-persist/integration/react";
import persistStore from "redux-persist/es/persistStore";
import { SplashScreen } from "@fluster/webview_utils";
import { GlobalStateInitializer } from "./global_state_initializer";

interface Props {
    children: ReactNode;
}

const ReduxProvider = ({ children }: Props) => {
    const persistor = persistStore(store);

    return (
        <Provider store={store}>
            <PersistGate
                loading={<SplashScreen message="Loading initial state..." />}
                persistor={persistor}
            >
                <>
                    <GlobalStateInitializer />
                    {children}
                </>
            </PersistGate>
        </Provider>
    );
};

ReduxProvider.displayName = "ReduxProvider";

export default ReduxProvider;
