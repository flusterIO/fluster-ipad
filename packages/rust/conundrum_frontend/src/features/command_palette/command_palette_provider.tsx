"use client";
import { type ReactNode, createContext, useReducer, useContext } from "react";
import { type CommandPaletteCommand } from "./commands/command_palette_command";
import { HomeCOmmandPaletteCommand as HomeCommandPaletteCommand } from "./commands/home_command";
import { type CommandGroupId } from "./commands/command_group_id";

export type ChildCommandRecord = Record<
    CommandGroupId | "unknown",
    CommandPaletteCommand[]
> & { isEmpty: boolean };

export interface CommandPaletteState {
    /**
     * The tree of sorts of the selected commands.
     */
    commands: CommandPaletteCommand[];
    /**
     * The child commands of the currently active command.
     * It will be undefined while the command is loading it's children.
     */
    childCommands?: Partial<ChildCommandRecord>;
}

const defaultInitialValues: CommandPaletteState = {
    commands: [],
    childCommands: undefined,
};

export const CommandPaletteContext =
    createContext<CommandPaletteState>(defaultInitialValues);

type CommandPaletteContextActions =
    | {
        type: "appendCommand";
        payload: CommandPaletteCommand;
    }
    | {
        type: "popLastCommand";
        payload?: undefined;
    }
    | {
        type: "setChildCommands";
        payload: Partial<ChildCommandRecord>;
    }
    | {
        type: "closeCommandPalette";
        payload?: undefined;
    }
    | {
        type: "openCommandPalette";
        payload?: undefined;
    };

export const CommandPaletteDispatchContext = createContext<
    React.Dispatch<CommandPaletteContextActions>
>(null!);

export const useCommandPaletteContext = () => useContext(CommandPaletteContext);
export const useCommandPaletteDispatch = () =>
    useContext(CommandPaletteDispatchContext);

export const CommandPaletteContextReducer = (
    state: CommandPaletteState,
    action: CommandPaletteContextActions,
): CommandPaletteState => {
    switch (action.type) {
        case "appendCommand": {
            return {
                ...state,
                commands: [...state.commands, action.payload],
            };
        }
        case "openCommandPalette": {
            return {
                ...state,
                commands: state.commands.length
                    ? state.commands
                    : [new HomeCommandPaletteCommand()],
            };
        }
        case "closeCommandPalette": {
            return {
                ...state,
                commands: [],
                childCommands: undefined,
            };
        }
        case "popLastCommand": {
            return {
                ...state,
                commands: state.commands.filter((_, i, a) => i !== a.length - 1),
            };
        }
        case "setChildCommands": {
            return {
                ...state,
                childCommands: action.payload,
            };
        }
        default: {
            return state;
        }
    }
};

CommandPaletteContextReducer.displayName = "CommandPaletteContextReducer";

interface CommandPaletteProviderProps {
    children: ReactNode;
    initialValues?: Partial<CommandPaletteState>;
}

export const CommandPaletteProvider = ({
    children,
    initialValues,
}: CommandPaletteProviderProps) => {
    const [state, dispatch] = useReducer(
        CommandPaletteContextReducer,
        initialValues
            ? { ...initialValues, ...defaultInitialValues }
            : defaultInitialValues,
    );

    return (
        <CommandPaletteContext.Provider value={state}>
            <CommandPaletteDispatchContext.Provider value={dispatch}>
                {children}
            </CommandPaletteDispatchContext.Provider>
        </CommandPaletteContext.Provider>
    );
};
