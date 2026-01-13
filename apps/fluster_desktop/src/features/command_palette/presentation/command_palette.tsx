import React, {
  useEffect,
  useEffectEvent,
  useMemo,
  useRef,
  useState,
  type ReactNode,
} from "react";

import { AppState } from "@/state/initial_state";
import { connect, useDispatch } from "react-redux";
import {
  InputGroup,
  InputGroupAddon,
  InputGroupInput,
  Kbd,
  KbdGroup,
  smartCaseContains,
} from "@fluster/webview_utils";
import { Search } from "lucide-react";
import { getCommandPaletteRoot } from "../data/command_palette_tree/command_palette_root";
import { CommandPaletteItem } from "../data/command_palette_item";
import { CommandPaletteItemComponent } from "./command_palette_item";
import { setCommandPaleteOpen } from "../state/command_palette_state_slice";

const connector = connect((state: AppState) => ({
  open: state.commandPalette.open,
}));

interface CommandPaletteProps {
  open: boolean;
}

export const CommandPalette = connector(
  ({ open }: CommandPaletteProps): ReactNode => {
    const dispatch = useDispatch();
    const [focusedIndex, setFocusedIndex] = useState(0);
    const [query, setQuery] = useState("");
    const [tree, setTree] = useState<CommandPaletteItem[]>([
      getCommandPaletteRoot(),
    ]);
    const [children, setChildren] = useState<CommandPaletteItem[]>([]);
    const ref = useRef<HTMLInputElement>(null);

    const filteredChildren = useMemo(() => {
      return children.filter((f) => smartCaseContains(f.label, query));
    }, [children, query]);

    const handleChildren = useEffectEvent(async () => {
      const selectedItem = tree[tree.length - 1];
      if (selectedItem?.children) {
        const res = await selectedItem.children();
        setChildren(res);
      }
    });

    useEffect(() => {
      handleChildren();
    }, [tree]);

    useEffect(() => {
      if (open) {
        ref.current?.focus();
      }
    }, [open]);

    if (!open) {
      return null;
    }

    const incrementFocus = (): void => {
      if (focusedIndex <= filteredChildren.length - 2) {
        setFocusedIndex(focusedIndex + 1);
      } else {
        setFocusedIndex(0);
      }
    };
    const decrementFocus = (): void => {
      if (focusedIndex <= 0) {
        setFocusedIndex(filteredChildren.length - 1);
      } else {
        setFocusedIndex(focusedIndex - 1);
      }
    };

    return (
      <div
        className="w-full max-w-[min(90vw,768px)] max-h-[min(768px,70vh)] bg-card border rounded"
        onClick={(e) => {
          e.stopPropagation();
        }}
      >
        <InputGroup className="has-[[data-slot=input-group-control]:focus-visible]:outline-none has-[[data-slot=input-group-control]:focus-visible]:ring-transparent has-[[data-slot=input-group-control]:focus-visible]:shadow-none has-[[data-slot=input-group-control]:focus-visible]:border-t-transparent has-[[data-slot=input-group-control]:focus-visible]:border-l-transparent has-[[data-slot=input-group-control]:focus-visible]:border-r-transparent has-[[data-slot=input-group-control]:focus-visible]:border-b-primary rounded-bl-none rounded-br-none">
          <InputGroupInput
            ref={ref}
            value={query}
            onChange={(e) => {
              setQuery(e.target.value);
              setFocusedIndex(0);
            }}
            className="focus-visible:outline-none focus-visible:border-transparent focus-visible:ring-none"
            placeholder="Search..."
            onKeyDown={(e) => {
              if (e.key === "Enter") {
                e.stopPropagation();
                e.preventDefault();
                const selectedItem = filteredChildren[focusedIndex];
                if (selectedItem?.action) {
                  selectedItem.action(dispatch);
                  dispatch(setCommandPaleteOpen(false));
                } else if (selectedItem.children) {
                  setTree([...tree, selectedItem]);
                }
              } else if (e.key === "Tab") {
                e.stopPropagation();
                e.preventDefault();
                if (e.shiftKey) {
                  decrementFocus();
                } else {
                  incrementFocus();
                }
              }
            }}
          />
          <InputGroupAddon>
            <Search />
          </InputGroupAddon>
          <InputGroupAddon align="inline-end">
            <Kbd>⌘</Kbd>
            <Kbd>P</Kbd>
          </InputGroupAddon>
        </InputGroup>
        <div className="w-full flex flex-col justify-start items-center overflow-y-auto">
          {filteredChildren.length ? (
            filteredChildren.map((c, i) => {
              return (
                <CommandPaletteItemComponent
                  key={c.label}
                  idx={i}
                  item={c}
                  focusedIndex={focusedIndex}
                />
              );
            })
          ) : (
            <div className="w-full h-fit flex flex-col justify-center items-center py-4">
              No Results
            </div>
          )}
        </div>
        <div className="py-2 px-4 text-sm text-muted-foreground flex flex-row justify-between items-center bg-background">
          <KbdGroup>
            <Kbd>Tab</Kbd>
            <span>or</span>
            <Kbd>⬆</Kbd>
            <Kbd>⬇</Kbd>
            <span>to navigate</span>
          </KbdGroup>
          <KbdGroup>
            <Kbd>Enter</Kbd>
            <span>to select</span>
          </KbdGroup>
        </div>
      </div>
    );
  },
);

CommandPalette.displayName = "CommandPalette";
