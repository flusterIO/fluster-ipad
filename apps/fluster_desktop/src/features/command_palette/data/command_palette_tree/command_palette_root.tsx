import { ChevronRight } from "lucide-react";
import { CommandPaletteItem } from "../command_palette_item";
import { setPanelLeftOpen } from "#/panel_left/state/panel_left_slice";
import { setPanelRightOpen } from "#/panel_right/state/panel_right_slice";
import { IconBoxAlignLeft, IconBoxAlignRight } from "@tabler/icons-react";

export const getCommandPaletteRoot = () =>
    new CommandPaletteItem({
        label: "",
        icon: () => null,
        children: [
            new CommandPaletteItem({
                label: "Panel Left (toggle)",
                icon: IconBoxAlignLeft,
                action: (d) => d(setPanelLeftOpen("toggle")),
            }),
            new CommandPaletteItem({
                label: "Panel Right (toggle)",
                icon: IconBoxAlignRight,
                action: (d) => d(setPanelRightOpen("toggle")),
            }),
        ],
    });
