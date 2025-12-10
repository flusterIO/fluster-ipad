pub fn get_workspace_root() -> String {
    std::env::var("FLUSTER_IOS_ROOT")
        .expect("You must provide a FLUSTER_IOS_ROOT env variable set to the root of your workspace to continue.")
}
