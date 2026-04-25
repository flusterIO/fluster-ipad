use conundrum::lang::runtime::run_conundrum::{ParseConundrumOptions, run_conundrum};

use crate::runs_conundrum;

#[tokio::test]
pub async fn parses_grid_column_props() {
    let test_content = r#"
<Grid
    columns={2}
>
<Container>
   Container 1
</Container>
<Container>
   Container 2
</Container>
</Grid>
        "#;
    let res = run_conundrum(
           ParseConundrumOptions {
               content: test_content.to_string(),
               note_id: None,
               modifiers: Vec::new(),
               hide_components: Vec::new(),
               ..Default::default()
           }
       ).expect("Runs conundrum without throwing an error.");
    insta::assert_snapshot!(res.content);
}
