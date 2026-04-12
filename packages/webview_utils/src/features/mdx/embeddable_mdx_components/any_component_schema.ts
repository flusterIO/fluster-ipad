import { type aiNoteSummaryProps } from "#/ai/embeddable_components/ai_note_summary/ai_note_summary_props";
import type { admonitionPropsSchema } from "./admonition/admonition_props_schema";
import type { embeddableCardPropsSchema } from "./card/embeddable_card_props";
import type { embeddableUtiltyContainerProps } from "./container/embeddable_utility_container_props";
import { type emojiComponentProps } from "./emoji/emoji_component_props";
import type { embeddableResponsiveGridPropsSchema } from "./grid/embeddable_responsive_grid_props";
import type { embeddableHintComponentPropsSchema } from "./hint/hint_props_schema";
import type { hlPropsSchema } from "./hl/hl_props_schema";
import type { hrPropsSchema } from "./hr/hr_props_schema";
import { type equationReferencePropsSchema } from "./math/equation_reference/equation_reference_props";
import { type embeddableTabProps } from "./tabs/embeddable_tab_props";
import { type tabGroupComponentProps } from "./tabs/tab_group_props";
import { type tocProps } from "./toc/toc_props";
import type { ulPropsSchema } from "./ul/ul_props_schema";

export type AnyComponentSchema = typeof embeddableResponsiveGridPropsSchema | typeof embeddableUtiltyContainerProps | typeof hlPropsSchema | typeof ulPropsSchema | typeof admonitionPropsSchema | typeof embeddableCardPropsSchema | typeof hrPropsSchema | typeof embeddableHintComponentPropsSchema | typeof aiNoteSummaryProps | typeof tabGroupComponentProps | typeof embeddableTabProps | typeof equationReferencePropsSchema | typeof emojiComponentProps | typeof tocProps;
