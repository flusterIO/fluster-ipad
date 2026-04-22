use crate::render_tests::{render_test::render_test, write_test_ast::write_test_ast};

#[tokio::test]
async fn renders_tabgroup_to_html() {
    let test_content = r#"# My tabs 

Integer, tristique et, consequat cursus auctor dignissim eros tempus ipsum. Nam vulputate, tempus at pharetra in iaculis, malesuada nisl ultrices. Praesent et, nibh, consequat leo lectus imperdiet nisl pretium consequat. Nunc pulvinar eu dui, nam leo velit non, enim nullam. Aliquam dui metus, felis tortor mus at nec vel, ac.


Praesent diam nunc, eu, turpis nec aliquet proin eget rhoncus. Eleifend eu, at nulla rhoncus varius fermentum, morbi arcu urna. Lacus ultrices arcu sed aenean velit montes rutrum nisi consequat. Felis, vulputate faucibus, quam praesent imperdiet aenean non in tempor. Fermentum, adipiscing sit ut dignissim in mus iaculis, etiam sapien.


Nam a, nullam tristique nunc aliquam suspendisse ligula, efficitur massa. Sem in diam dictumst in enim condimentum, nunc diam feugiat. Nulla, imperdiet urna consectetur, neque lobortis nunc quis consequat dapibus. Ac laoreet, ultricies, enim pellentesque pellentesque pellentesque vulputate consectetur eu. Tempor nam eu, dolor consectetur, vitae enim efficitur in faucibus.

<Tabs research>
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
</Tabs>

Nam a, nullam tristique nunc aliquam suspendisse ligula, efficitur massa. Sem in diam dictumst in enim condimentum, nunc diam feugiat. Nulla, imperdiet urna consectetur, neque lobortis nunc quis consequat dapibus. Ac laoreet, ultricies, enim pellentesque pellentesque pellentesque vulputate consectetur eu. Tempor nam eu, dolor consectetur, vitae enim efficitur in faucibus.


Nam a, nullam tristique nunc aliquam suspendisse ligula, efficitur massa. Sem in diam dictumst in enim condimentum, nunc diam feugiat. Nulla, imperdiet urna consectetur, neque lobortis nunc quis consequat dapibus. Ac laoreet, ultricies, enim pellentesque pellentesque pellentesque vulputate consectetur eu. Tempor nam eu, dolor consectetur, vitae enim efficitur in faucibus.
"#;
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
    write_test_ast(clean_test, "Tab Group");
    render_test(clean_test, "tabs-group").await;
}
