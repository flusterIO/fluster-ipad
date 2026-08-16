import { rspc } from "@/app/rspc_client";
import { useEffect } from "react";
import { executeKeyboardAction } from "./execute_keyboard_action";

export const useGlobalKeyboardListener = () => {
    const { data: shortcuts = [] } = rspc.useQuery([
        "crud.keyboard_shortcut.get_by_predicate",
        {
            pagination: {
                page: 1,
                per_page: 999,
            },
            predicate: null,
            sort: null,
        },
    ]);

    const handleKeyDown = (e: KeyboardEvent) => {
        for (const k of shortcuts) {
            if (
                k.key === e.key &&
                k.meta === e.metaKey &&
                k.alt === e.altKey &&
                k.ctrl === e.ctrlKey
            ) {
                e.stopPropagation();
                e.preventDefault();
                executeKeyboardAction(k.action);
                return;
            }
        }
    };

    useEffect(() => {
        window.addEventListener("keydown", handleKeyDown);
        return () => {
            window.removeEventListener("keydown", handleKeyDown);
        };
    }, []);

    return shortcuts;
};
