import { useEffect } from "react";

export const useRequestDocumentSize = () => {
    useEffect(() => {
        window.requestDocumentSize();
    }, []);
};
