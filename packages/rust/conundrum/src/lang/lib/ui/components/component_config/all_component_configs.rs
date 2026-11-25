use crate::lang::lib::ui::components::{
    component_config::component_config::ComponentConfig, media::video::youtube::youtube_config::youtube_component_config,
};

pub fn get_all_component_configs() -> Vec<ComponentConfig> {
    let items: Vec<ComponentConfig> = vec![youtube_component_config()];
    items
}
