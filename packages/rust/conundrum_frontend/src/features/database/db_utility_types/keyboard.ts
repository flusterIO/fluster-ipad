import { type Procedures } from "@/codegen/bindings";

export type KeyboardShortcutResult =
    Procedures["crud"]["keyboard_shortcut"]["get_by_predicate"]["output"][number];

export type KeyboardShortcutInput =
    Procedures["crud"]["keyboard_shortcut"]["save_many"]["input"];

export type KeyboardShortcutItem = KeyboardShortcutInput[number];
