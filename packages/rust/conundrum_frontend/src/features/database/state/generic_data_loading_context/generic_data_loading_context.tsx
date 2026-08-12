"use client";
import React, {
    type ReactNode,
    createContext,
    useReducer,
    useContext,
    useEffect,
    useEffectEvent,
} from "react";
import consola from "consola";
import { logMaybeObject } from "#/error_handling/utils/log_maybe_object";

export interface GenericRemoteDataState<GenericDataType> {
    data: GenericDataType | null;
    loading: boolean;
}

export const GenericRemoteDataContext = createContext<
    GenericRemoteDataState<unknown>
>({
    data: null,
    loading: true,
});

type GenericRemoteDataContextActions<GenericDataType> =
    | { type: "set-data"; payload: GenericDataType | null }
    | {
        type: "clear-data";
        payload?: undefined;
    }
    | {
        type: "set-loading";
        payload: boolean | "toggle";
    };

export const GenericRemoteDataDispatchContext = createContext<
    React.Dispatch<GenericRemoteDataContextActions<unknown>>
>(null!);

export const useGenericRemoteDataContext = <
    // eslint-disable-next-line @typescript-eslint/no-unnecessary-type-constraint
    GenericDataType extends unknown,
>() =>
    useContext(
        GenericRemoteDataContext,
    ) as GenericRemoteDataState<GenericDataType>;

export const useGenericRemoteDataDispatch = <
    // eslint-disable-next-line @typescript-eslint/no-unnecessary-type-constraint
    GenericDataType extends unknown,
>() =>
    useContext(GenericRemoteDataDispatchContext) as React.Dispatch<
        GenericRemoteDataContextActions<GenericDataType>
    >;

export const GenericRemoteDataContextReducer = <
    // eslint-disable-next-line @typescript-eslint/no-unnecessary-type-constraint
    GenericDataType extends unknown,
>(
    state: GenericRemoteDataState<GenericDataType>,
    action: GenericRemoteDataContextActions<GenericDataType>,
): GenericRemoteDataState<GenericDataType> => {
    switch (action.type) {
        case "clear-data": {
            return {
                ...state,
                data: null,
            };
        }
        case "set-data": {
            return {
                ...state,
                data: action.payload,
                loading: !action.payload,
            };
        }
        case "set-loading": {
            return {
                ...state,
                loading: action.payload === "toggle" ? !state.loading : action.payload,
            };
        }
        default: {
            return state;
        }
    }
};

GenericRemoteDataContextReducer.displayName = "GenericRemoteDataContextReducer";

interface GenericRemoteDataProviderProps<GenericDataType> {
    children: ReactNode;
    initialValues: GenericRemoteDataState<GenericDataType>;
    loader: () => Promise<GenericDataType | null>;
    /**
     * An optional key that will trigger a refetch of the data.
     */
    dataKey?: string;
}

// eslint-disable-next-line @typescript-eslint/no-unnecessary-type-constraint
export const GenericRemoteDataProvider = <GenericDataType extends unknown>({
    children,
    dataKey,
    initialValues,
    loader,
}: GenericRemoteDataProviderProps<GenericDataType>) => {
    const [state, dispatch] = useReducer(
        GenericRemoteDataContextReducer,
        initialValues,
    );

    const getData = useEffectEvent(async () => {
        const res = await loader();
        if (res) {
            dispatch({
                type: "set-data",
                payload: res,
            });
        } else {
            consola.warn("Failed to fetch data.");
        }
    });

    useEffect(() => {
        getData().catch((err: unknown) => {
            logMaybeObject("Error: ", err);
        });
    }, [loader, dataKey]);

    return (
        <GenericRemoteDataContext.Provider value={state}>
            <GenericRemoteDataDispatchContext.Provider value={dispatch}>
                {children}
            </GenericRemoteDataDispatchContext.Provider>
        </GenericRemoteDataContext.Provider>
    );
};
