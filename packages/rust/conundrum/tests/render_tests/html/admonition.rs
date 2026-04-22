use crate::render_tests::render_test::render_test;

#[tokio::test]
async fn renders_admonitions_to_html() {
    let test_content = r#"# My title
<Admonition title="My admonition">
Turpis suscipit dapibus feugiat nunc lorem ac consectetur convallis neque. Id turpis arcu eget convallis vulputate condimentum, vivamus nulla pellentesque. Dignissim suscipit, arcu vulputate bibendum penatibus sed tortor dolor eget. Justo vel dignissim lectus euismod, pellentesque pulvinar, facilisis varius consequat. Amet massa in consequat elit diam tortor massa id tellus.

Metus non aliquam consequat, hac porta lectus sagittis nunc faucibus. Amet nec tristique dignissim nulla habitasse mi eget, ridiculus, nunc. In nullam dictum, diam ac consectetur dui pharetra enim vestibulum. At facilisis dictum sed porta mi sem convallis dictum, ornare. Praesent tristique ut eget, dapibus tempor imperdiet consequat nunc, ut.

Pellentesque aliquam, neque facilisis molestie tempor, nulla nullam et nunc. Cras tristique faucibus facilisis ullamcorper elementum duis sed dui, nibh. Vel, tempor lorem dignissim dignissim varius lectus et tempor, tempor. Orci, elementum faucibus felis, velit lacus fermentum elit ultricies consequat. Pharetra lacus fringilla ac imperdiet fermentum duis, auctor fermentum, etiam.
</Admonition>

<Admonition title="My Foldable Admonition" foldable>
Turpis suscipit dapibus feugiat nunc lorem ac consectetur convallis neque. Id turpis arcu eget convallis vulputate condimentum, vivamus nulla pellentesque. Dignissim suscipit, arcu vulputate bibendum penatibus sed tortor dolor eget. Justo vel dignissim lectus euismod, pellentesque pulvinar, facilisis varius consequat. Amet massa in consequat elit diam tortor massa id tellus.

Metus non aliquam consequat, hac porta lectus sagittis nunc faucibus. Amet nec tristique dignissim nulla habitasse mi eget, ridiculus, nunc. In nullam dictum, diam ac consectetur dui pharetra enim vestibulum. At facilisis dictum sed porta mi sem convallis dictum, ornare. Praesent tristique ut eget, dapibus tempor imperdiet consequat nunc, ut.

Pellentesque aliquam, neque facilisis molestie tempor, nulla nullam et nunc. Cras tristique faucibus facilisis ullamcorper elementum duis sed dui, nibh. Vel, tempor lorem dignissim dignissim varius lectus et tempor, tempor. Orci, elementum faucibus felis, velit lacus fermentum elit ultricies consequat. Pharetra lacus fringilla ac imperdiet fermentum duis, auctor fermentum, etiam.

</Admonition>

Turpis suscipit dapibus feugiat nunc lorem ac consectetur convallis neque. Id turpis arcu eget convallis vulputate condimentum, vivamus nulla pellentesque. Dignissim suscipit, arcu vulputate bibendum penatibus sed tortor dolor eget. Justo vel dignissim lectus euismod, pellentesque pulvinar, facilisis varius consequat. Amet massa in consequat elit diam tortor massa id tellus.

<Admonition title="My sidebar admonition" sidebar right>
Turpis suscipit dapibus feugiat nunc lorem ac consectetur convallis neque. Id turpis arcu eget convallis vulputate condimentum, vivamus nulla pellentesque. Dignissim suscipit, arcu vulputate bibendum penatibus sed tortor dolor eget. Justo vel dignissim lectus euismod, pellentesque pulvinar, facilisis varius consequat. Amet massa in consequat elit diam tortor massa id tellus.
</Admonition>

Metus non aliquam consequat, hac porta lectus sagittis nunc faucibus. Amet nec tristique dignissim nulla habitasse mi eget, ridiculus, nunc. In nullam dictum, diam ac consectetur dui pharetra enim vestibulum. At facilisis dictum sed porta mi sem convallis dictum, ornare. Praesent tristique ut eget, dapibus tempor imperdiet consequat nunc, ut.

Pellentesque aliquam, neque facilisis molestie tempor, nulla nullam et nunc. Cras tristique faucibus facilisis ullamcorper elementum duis sed dui, nibh. Vel, tempor lorem dignissim dignissim varius lectus et tempor, tempor. Orci, elementum faucibus felis, velit lacus fermentum elit ultricies consequat. Pharetra lacus fringilla ac imperdiet fermentum duis, auctor fermentum, etiam.


<Admonition title="My Folded Admonition" folded>
Turpis suscipit dapibus feugiat nunc lorem ac consectetur convallis neque. Id turpis arcu eget convallis vulputate condimentum, vivamus nulla pellentesque. Dignissim suscipit, arcu vulputate bibendum penatibus sed tortor dolor eget. Justo vel dignissim lectus euismod, pellentesque pulvinar, facilisis varius consequat. Amet massa in consequat elit diam tortor massa id tellus.

Metus non aliquam consequat, hac porta lectus sagittis nunc faucibus. Amet nec tristique dignissim nulla habitasse mi eget, ridiculus, nunc. In nullam dictum, diam ac consectetur dui pharetra enim vestibulum. At facilisis dictum sed porta mi sem convallis dictum, ornare. Praesent tristique ut eget, dapibus tempor imperdiet consequat nunc, ut.

Pellentesque aliquam, neque facilisis molestie tempor, nulla nullam et nunc. Cras tristique faucibus facilisis ullamcorper elementum duis sed dui, nibh. Vel, tempor lorem dignissim dignissim varius lectus et tempor, tempor. Orci, elementum faucibus felis, velit lacus fermentum elit ultricies consequat. Pharetra lacus fringilla ac imperdiet fermentum duis, auctor fermentum, etiam.
</Admonition>

<Admonition title="My Folded & Foldable Admonition" folded foldable>
Turpis suscipit dapibus feugiat nunc lorem ac consectetur convallis neque. Id turpis arcu eget convallis vulputate condimentum, vivamus nulla pellentesque. Dignissim suscipit, arcu vulputate bibendum penatibus sed tortor dolor eget. Justo vel dignissim lectus euismod, pellentesque pulvinar, facilisis varius consequat. Amet massa in consequat elit diam tortor massa id tellus.

Metus non aliquam consequat, hac porta lectus sagittis nunc faucibus. Amet nec tristique dignissim nulla habitasse mi eget, ridiculus, nunc. In nullam dictum, diam ac consectetur dui pharetra enim vestibulum. At facilisis dictum sed porta mi sem convallis dictum, ornare. Praesent tristique ut eget, dapibus tempor imperdiet consequat nunc, ut.

Pellentesque aliquam, neque facilisis molestie tempor, nulla nullam et nunc. Cras tristique faucibus facilisis ullamcorper elementum duis sed dui, nibh. Vel, tempor lorem dignissim dignissim varius lectus et tempor, tempor. Orci, elementum faucibus felis, velit lacus fermentum elit ultricies consequat. Pharetra lacus fringilla ac imperdiet fermentum duis, auctor fermentum, etiam.
</Admonition>
    "#;
    render_test(test_content, "Admonition").await;
}
