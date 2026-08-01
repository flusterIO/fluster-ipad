import React, {
    useEffect,
    useEffectEvent,
    useState,
    type ReactNode,
} from "react";
import {
    Command,
    CommandItem,
    CommandDialog,
    CommandInput,
    CommandSeparator,
    CommandList,
    CommandEmpty,
    CommandGroup,
    CommandShortcut,
} from "@/components/shad/command";
import { connect, useDispatch } from "react-redux";
import { type AppState } from "@/state/initial_state";
import consola from "consola";

import { setCommandPaletteOpen } from "#/navigation/state/navigation_slice";
import { type CommandPaletteCommand } from "./commands/command_palette_command";
import {
    SyncFileSystemWithAICommand,
    SyncFileSystemWithoutAICommand,
} from "./commands/sync/sync_filesystem";

const connector = connect((state: AppState) => ({
    commands: state.navigation.commandPalette,
}));

const CommandPaletteItemFromClass = ({
    item,
}: {
    item: CommandPaletteCommand;
}): ReactNode => {
    return <CommandItem>{item.label}</CommandItem>;
};

type ChildCommandRecord = Record<string | "unknown", CommandPaletteCommand[]>;

export const CommandPalette = connector(
    ({
        commands,
    }: {
        commands: AppState["navigation"]["commandPalette"];
    }): ReactNode => {
        const [childCommands, setChildCommands] = useState<ChildCommandRecord>({});
        const [inputValue, setInputValue] = useState("");
        const dispatch = useDispatch();

        const handleChildCommands = useEffectEvent(async () => {
            if (!commands.length) {
                return;
            }
            const res = await commands[0].children();
            const items: ChildCommandRecord = {};
            for (const k of res) {
                if (k.groupId) {
                    if (!items[k.groupId]) {
                        items[k.groupId] = [];
                    }
                    items[k.groupId].push(k);
                } else {
                    if (!items.unknown) {
                        items.unknown = [];
                    }
                    items.unknown.push(k);
                }
            }
            setChildCommands(items);
        });

        useEffect(() => {
            handleChildCommands().catch((err: unknown) => {
                consola.error(`Error: ${err}`);
            });
        }, [commands]);

        return (
            <CommandDialog
                title="Commands"
                open={Boolean(commands.length)}
                onOpenChange={(b) => {
                    if (!b) {
                        dispatch(setCommandPaletteOpen(false));
                    }
                }}
            >
                <Command>
                    <CommandInput value={inputValue} onValueChange={setInputValue} />
                    <CommandList>
                        <CommandEmpty>No results to show</CommandEmpty>
                        {Object.keys(childCommands).map((k) => {
                            const items = childCommands[k];
                            if (k !== "unknown") {
                                return (
                                    <CommandGroup>
                                        {items.map((kk) => {
                                            return <CommandItem>{kk.label}</CommandItem>;
                                        })}
                                    </CommandGroup>
                                );
                            } else {
                                return (
                                    <>
                                        {items.map((kk) => {
                                            return <CommandItem>{kk.label}</CommandItem>;
                                        })}
                                    </>
                                );
                            }
                        })}
                    </CommandList>
                </Command>
            </CommandDialog>
        );
    },
);

CommandPalette.displayName = "CommandPalette";
