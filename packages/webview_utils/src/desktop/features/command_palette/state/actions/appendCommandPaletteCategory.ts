import { Location } from "react-router";
import {
    CommandPaletteActionType,
    CommandPaletteContextActions,
} from "../command_palette_provider";
import { Dispatch } from "react";
import { CommandPaletteCategory } from "../../data/models/command_palette_category";

export const appendCommandPaletteCategory = async (
    cat: CommandPaletteCategory,
    location: Location,
    dispatch: Dispatch<CommandPaletteContextActions>
): Promise<void> => {
    const items = await cat.getItems(location);
    dispatch({
        type: CommandPaletteActionType.appendCommandPaletteCategory,
        payload: {
            cat: cat,
            items: items,
        },
    });
};
