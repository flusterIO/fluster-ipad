/* eslint-disable @typescript-eslint/prefer-nullish-coalescing */
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
    CommandList,
    CommandEmpty,
    CommandGroup,
} from "@/components/shad/command";
import consola from "consola";

import {
    type ChildCommandRecord,
    useCommandPaletteContext,
    useCommandPaletteDispatch,
} from "./command_palette_provider";
import {
    type CommandGroupId,
    commandGroupIdToGroupLabel,
    commandPaletteGroupIdToKeywords,
} from "./commands/command_group_id";
import { type CommandPaletteCommand } from "./commands/command_palette_command";
import { useNavigate } from "react-router";

const CI = ({
    item,
    clearInput,
}: {
    item: CommandPaletteCommand;
    clearInput: () => void;
}): ReactNode => {
    const dispatch = useCommandPaletteDispatch();
    const navigate = useNavigate()
    return (
        <CommandItem
            keywords={commandPaletteGroupIdToKeywords(item.groupId, item.keywords)}
            onSelect={() => {
                if (item.hasChildren) {
                    dispatch({
                        type: "appendCommand",
                        payload: item,
                    });
                    clearInput();
                } else {
                    (async () => {
                        await item.act(navigate);
                        dispatch({
                            type: "closeCommandPalette",
                            payload: undefined,
                        });
                    })().catch((err: unknown) => {
                        consola.error(`Error: ${err}`);
                    });
                }
            }}
        >
            {item.label}
        </CommandItem>
    );
};

export const CommandPalette = (): ReactNode => {
    const [inputValue, setInputValue] = useState("");
    const dispatch = useCommandPaletteDispatch();
    const { commands, childCommands } = useCommandPaletteContext();

    const handleChildCommands = useEffectEvent(async () => {
        const res = await commands[commands.length - 1].children();
        const items: Partial<ChildCommandRecord> = {
            isEmpty: res.length <= 0,
        };
        for (const k of res) {
            if (k.groupId) {
                if (!items[k.groupId]) {
                    items[k.groupId] = [];
                }

                /* @ts-expect-error -- It'll be there */
                items[k.groupId].push(k);
            } else {
                if (!items.unknown) {
                    items.unknown = [];
                }
                items.unknown.push(k);
            }
        }
        dispatch({
            type: "setChildCommands",
            payload: items,
        });
    });

    useEffect(() => {
        if (!commands.length) {
            return;
        } else {
            handleChildCommands().catch((err: unknown) => {
                consola.error(`Error: `, err);
            });
        }
    }, [commands]);

    return (
        <CommandDialog
            title="Commands"
            open={Boolean(commands.length)}
            onOpenChange={(b) => {
                if (!b) {
                    dispatch({
                        type: "closeCommandPalette",
                    });
                }
            }}
        >
            <Command vimBindings autoFocus loop>
                <CommandInput
                    value={inputValue}
                    onValueChange={setInputValue}
                    onKeyDown={(e) => {
                        if (
                            e.key === "Backspace" &&
                            (e.target as HTMLInputElement).value === "" &&
                            commands.length > 1
                        ) {
                            dispatch({
                                type: "popLastCommand",
                            });
                        }
                    }}
                />
                <CommandList>
                    {childCommands?.isEmpty || !childCommands ? (
                        <CommandEmpty>No results to show</CommandEmpty>
                    ) : (
                        Object.keys(childCommands).map((k) => {
                            if (k === "isEmpty") {
                                return null;
                            }
                            const items = childCommands[k as CommandGroupId] ?? [];
                            if (k !== "unknown") {
                                return (
                                    <CommandGroup>
                                        <div className="text-bold text-sm mb-2 mt-0 flex flex-row justify-start items-center w-full gap-x-2">
                                            <div className="w-4 h-0.5 bg-muted" />
                                            <div className="text-muted-foreground!">
                                                {commandGroupIdToGroupLabel(k as CommandGroupId)}
                                            </div>
                                            <div className="bg-muted h-0.5 grow" />
                                        </div>
                                        {items.map((kk) => {
                                            return (
                                                <CI
                                                    clearInput={() => {
                                                        setInputValue("");
                                                    }}
                                                    item={kk}
                                                    key={kk.key}
                                                />
                                            );
                                        })}
                                    </CommandGroup>
                                );
                            } else {
                                return (
                                    <>
                                        {items.map((kk) => {
                                            return (
                                                <CI
                                                    clearInput={() => {
                                                        setInputValue("");
                                                    }}
                                                    item={kk}
                                                    key={kk.key}
                                                />
                                            );
                                        })}
                                    </>
                                );
                            }
                        })
                    )}
                </CommandList>
            </Command>
        </CommandDialog>
    );
};

CommandPalette.displayName = "CommandPalette";
