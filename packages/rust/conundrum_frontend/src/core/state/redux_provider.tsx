import React, { type ReactNode } from "react";
import store from "./store";
import { Provider } from "react-redux";
import { PersistGate } from "redux-persist/integration/react";
import persistStore from "redux-persist/es/persistStore";
import { FullScreenLoading } from "#/navigation/full_screen_loading";

const ReduxProvider = ({ children }: { children: ReactNode }) => {
    const persistor = persistStore(store);

    return (
        <Provider store={store}>
            <PersistGate
                loading={<FullScreenLoading message="Loading saved data..." />}
                persistor={persistor}
            >
                <>{children}</>
            </PersistGate>
        </Provider>
    );
};

ReduxProvider.displayName = "ReduxProvider";

export default ReduxProvider;
