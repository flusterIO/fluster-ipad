pub async fn path_exists(route_path: &str) -> bool {
    tokio::fs::try_exists(route_path).await.is_ok_and(|n| n)
}
