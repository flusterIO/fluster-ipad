import type { admonitionPropsSchema } from "./admonition/admonition_props_schema";
import type { embeddableCardPropsSchema } from "./card/embeddable_card_props";
import type { embeddableUtiltyContainerProps } from "./container/embeddable_utility_container_props";
import type { embeddableResponsiveGridPropsSchema } from "./grid/embeddable_responsive_grid_props";
import type { embeddableHintComponentPropsSchema } from "./hint/hint_props_schema";
import type { hlPropsSchema } from "./hl/hl_props_schema";
import type { hrPropsSchema } from "./hr/hr_props_schema";
import type { ulPropsSchema } from "./ul/ul_props_schema";

export type AnyComponentSchema = typeof embeddableResponsiveGridPropsSchema | typeof embeddableUtiltyContainerProps | typeof hlPropsSchema | typeof ulPropsSchema | typeof admonitionPropsSchema | typeof embeddableCardPropsSchema | typeof hrPropsSchema | typeof embeddableHintComponentPropsSchema
