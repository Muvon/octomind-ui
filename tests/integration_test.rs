#[tokio::test]
async fn test_get_sessions() {
    let result = octomind_ui::commands::get_available_sessions().await;
    println!("Available sessions: {:?}", result);
    // This should not fail even if no sessions exist
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_list_directories() {
    let result = octomind_ui::commands::list_directories().await;
    println!("Available directories: {:?}", result);
    assert!(result.is_ok());
    assert!(!result.unwrap().is_empty());
}
