import { useMemo } from "react";
import { useSearchParams } from "react-router";

export const useSearchParamsObject = (): Record<string, string> => {
    const [searchParams] = useSearchParams();
    return useMemo(() => {
        const data = {};
        for (const k of searchParams?.entries()) {
            data[k[0]] = k[1];
        }
        return data;
    }, [searchParams]);
};
