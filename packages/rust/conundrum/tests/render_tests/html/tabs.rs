use conundrum::testing::render_test::render_test;

#[tokio::test]
async fn renders_tabgroup_to_html() {
    let clean_test = r#"<Tabs research>
<Tab label="My tab one">
My tab here
</Tab>
<Tab label="My tab two">
My tab two with a much longer bodyDui non ac aliquam, convallis velit, eros dignissim ex elementum. Feugiat et faucibus vitae lorem neque dui in nunc, convallis. Sapien pretium faucibus, eleifend sed dolor eleifend arcu, ultricies tincidunt. Neque dapibus parturient dapibus dictum imperdiet nisi eros habitasse ac. Morbi et faucibus amet eu suscipit, dui ac vestibulum, nunc.Venenatis dui enim sagittis in tellus hac arcu enim nibh. Dictum consequat praesent sed in maecenas enim, sed arcu fermentum. Nec dolor, non in metus, nec fusce suspendisse in et. Id vitae tempus ac lorem faucibus habitasse imperdiet, efficitur, velit. Pellentesque arcu suspendisse, nam id eu aenean amet eros ac.Elementum venenatis posuere habitasse eleifend ante, iaculis duis magna pellentesque. Condimentum, leo posuere et nulla, vestibulum nibh donec ullamcorper quis. Vitae, diam ut, integer at posuere tortor semper in enim. Pretium, praesent aliquet nunc viverra a, tristique neque lacus faucibus. Dui, volutpat laoreet suspendisse nunc magna eget consectetur ac, eget. 
My tab two with a much longer bodyDui non ac aliquam, convallis velit, eros dignissim ex elementum. Feugiat et faucibus vitae lorem neque dui in nunc, convallis. Sapien pretium faucibus, eleifend sed dolor eleifend arcu, ultricies tincidunt. Neque dapibus parturient dapibus dictum imperdiet nisi eros habitasse ac. Morbi et faucibus amet eu suscipit, dui ac vestibulum, nunc.Venenatis dui enim sagittis in tellus hac arcu enim nibh. Dictum consequat praesent sed in maecenas enim, sed arcu fermentum. Nec dolor, non in metus, nec fusce suspendisse in et. Id vitae tempus ac lorem faucibus habitasse imperdiet, efficitur, velit. Pellentesque arcu suspendisse, nam id eu aenean amet eros ac.Elementum venenatis posuere habitasse eleifend ante, iaculis duis magna pellentesque. Condimentum, leo posuere et nulla, vestibulum nibh donec ullamcorper quis. Vitae, diam ut, integer at posuere tortor semper in enim. Pretium, praesent aliquet nunc viverra a, tristique neque lacus faucibus. Dui, volutpat laoreet suspendisse nunc magna eget consectetur ac, eget. 
My tab two with a much longer bodyDui non ac aliquam, convallis velit, eros dignissim ex elementum. Feugiat et faucibus vitae lorem neque dui in nunc, convallis. Sapien pretium faucibus, eleifend sed dolor eleifend arcu, ultricies tincidunt. Neque dapibus parturient dapibus dictum imperdiet nisi eros habitasse ac. Morbi et faucibus amet eu suscipit, dui ac vestibulum, nunc.Venenatis dui enim sagittis in tellus hac arcu enim nibh. Dictum consequat praesent sed in maecenas enim, sed arcu fermentum. Nec dolor, non in metus, nec fusce suspendisse in et. Id vitae tempus ac lorem faucibus habitasse imperdiet, efficitur, velit. Pellentesque arcu suspendisse, nam id eu aenean amet eros ac.Elementum venenatis posuere habitasse eleifend ante, iaculis duis magna pellentesque. Condimentum, leo posuere et nulla, vestibulum nibh donec ullamcorper quis. Vitae, diam ut, integer at posuere tortor semper in enim. Pretium, praesent aliquet nunc viverra a, tristique neque lacus faucibus. Dui, volutpat laoreet suspendisse nunc magna eget consectetur ac, eget. 
</Tab>
<Tab label="My tab three">
My tab three
</Tab>
</Tabs>"#;
    render_test(clean_test, "tabs-group").await;
}
