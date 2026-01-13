import React, { type ReactNode } from "react";
import { ModalBackdrop } from "@fluster/webview_utils";
import { CommandPalette } from "./command_palette";

import { AppState } from "@/state/initial_state";
import { connect, useDispatch } from "react-redux";
import { setCommandPaleteOpen } from "../state/command_palette_state_slice";

const connector = connect((state: AppState) => ({
    open: state.commandPalette.open,
}));

interface Props {
    open: boolean;
}

export const CommandPaletteContainer = connector(
    ({ open }: Props): ReactNode => {
        const dispatch = useDispatch();

        return (
            <ModalBackdrop
                hide={!open}
                onClick={() => dispatch(setCommandPaleteOpen(false))}
                className="flex flex-col justify-start items-center py-24"
            >
                <CommandPalette />
            </ModalBackdrop>
        );
    },
);

CommandPaletteContainer.displayName = "CommandPaletteContainer";
