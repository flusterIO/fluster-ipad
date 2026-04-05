import { aiNoteSummaryComponentConfig } from "../../ai/embeddable_components/ai_note_summary/ai_note_summary_component_config";
import { admonitionComponentConfig } from "./admonition/admonition_component_config";
import { embeddableCardComponentConfig } from "./card/embeddable_card_component_config";
import { embeddableUtilityContainerComponentConfig } from "./container/embeddable_utility_container_component_config";
import { type EmbeddableComponentConfig } from "./embeddable_component_config";
import { embeddableGridComponentConfig } from "./grid/embeddable_responsive_grid_component_config";
import { embeddableHintComponentConfig } from "./hint/hint_component_config";
import { hlComponentConfig } from "./hl/hl_component_config";
import { hrComponentConfig } from "./hr/hr_component_config";
import { embeddableTabComponentConfig } from "./tabs/embeddable_tab_config";
import { tabGroupComponentConfig } from "./tabs/tab_group_component_config";
import { ulComponentConfig } from "./ul/ul_component_config";

export const embeddableComponentConfigs: EmbeddableComponentConfig[] = [
    admonitionComponentConfig,
    hlComponentConfig,
    ulComponentConfig,
    embeddableCardComponentConfig,
    embeddableUtilityContainerComponentConfig,
    embeddableGridComponentConfig,
    hrComponentConfig,
    embeddableHintComponentConfig,
    embeddableTabComponentConfig,
    tabGroupComponentConfig,
    aiNoteSummaryComponentConfig
]
