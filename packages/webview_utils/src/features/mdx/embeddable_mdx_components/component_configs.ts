import { admonitionComponentConfig } from "./admonition/admonition_component_config";
import { embeddableCardComponentConfig } from "./card/embeddable_card_component_config";
import { embeddableUtilityContainerComponentConfig } from "./container/embeddable_utility_container_component_config";
import { EmbeddableComponentConfig } from "./embeddable_component_config";
import { embeddableGridComponentConfig } from "./grid/embeddable_responsive_grid_component_config";
import { hlComponentConfig } from "./hl/hl_component_config";
import { ulComponentConfig } from "./ul/ul_component_config";

export const embeddableComponentConfigs: EmbeddableComponentConfig[] = [
    admonitionComponentConfig,
    hlComponentConfig,
    ulComponentConfig,
    embeddableCardComponentConfig,
    embeddableUtilityContainerComponentConfig,
    embeddableGridComponentConfig
]
