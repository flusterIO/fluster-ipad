use conundrum::testing::render_test::render_test;

#[tokio::test]
async fn renders_codeblock_to_html() {
    let test_content = r#"# My title 1 <Emoji smedium inline name="rocket"/>
> My subtitle

<Toc />

My paragraph goes here with inline $e=mc^2$ math.
My paragraph without a new line that makes me :smile:.

My <Ul>paragraph</Ul> _after_ **two** ***new lines***.  
My <Hl>paragraph after</Hl> two spaces and a line break.

My inline `const (x) => y / z;{:ts}` code here.

<Hint label="Happy :smile:">
My new hint goes here.
</Hint>

This is my [[#tagHere]] and my citation[[cite:Einstein]] here and [my note link](note:someFakeNoteId).

<Grid large={2} gap="small">
<Container centerContent error rounded="medium" padding="medium">
My item here
</Container>
<Container centerContent success rounded="medium" padding="medium">
My other item here
</Container>
</Grid>

## Some other 2 heading
> With a subtitle

### A level 3 heading

#### A level 4 heading

##### A level 5 heading

### A level 3 heading

<Tabs>
<Tab label="My tab one">
My tab here
</Tab>
<Tab label="My tab two">
My tab two
</Tab>
<Tab label="My tab three">
My tab three
</Tab>
</Tabs>

## A heading 2 without a subtitle

--- My hr with $\delta$ children here ---

<Admonition title="My admonition" foldable folded>
Vitae mi lacus, sed facilisis, sed montes dictum nibh sapien. Sed justo ante erat diam magna dapibus, convallis eget mi. Congue euismod, vestibulum posuere magna ut eu aenean euismod sit. Viverra tristique quis ac dapibus, pretium ridiculus morbi at, maecenas. Et praesent nec amet, vel vivamus feugiat, semper congue sit.

Nunc, habitasse dui diam nec duis eu eget nullam donec. Hac nunc, lectus sapien pellentesque laoreet elit pulvinar, etiam iaculis. Diam sem odio quisque massa ornare donec curabitur et ut. Elit lacus ante sed, metus ullamcorper turpis vulputate dignissim urna. Tortor nulla luctus, efficitur congue efficitur, euismod odio neque ultricies.

Lacus, ullamcorper, dictumst natoque auctor eleifend ipsum sit pellentesque convallis. Quisque ultricies, condimentum id ut a etiam, vel ac id. Ac in sed cursus congue suspendisse quis vel dictum tempor. Eu vestibulum, auctor magna sed eu sed eros dolor, ligula. Sit maecenas nulla, lacus eros dui arcu, sit suscipit in.

Augue enim morbi vestibulum iaculis metus, et finibus magnis facilisis. Aliquam porttitor ante est sed urna luctus aenean imperdiet aliquam. Maximus vitae convallis maecenas pharetra sit posuere sed, est, tincidunt. Lacus, tempor vulputate est massa, condimentum platea diam pulvinar rutrum. Vitae at, eu nulla id amet pulvinar velit tincidunt vitae.

Tortor aenean euismod, nisl dictum rutrum, vulputate auctor arcu est. Arcu quis adipiscing at, eleifend ac, amet dui lorem dignissim. Lectus justo sed et lacus lectus vel pellentesque aliquam nec. Sit ut, velit, id aliquam nam in ut sit at. Ac laoreet etiam at ac sem maecenas, ac sit, integer.
</Admonition>

<Card title="My card here" desc="My card's description" centerBody sidebar right>

$$
e=mc^2
$$

</Card>

> My block quote
> With a nested line
>> And a neted block quote.  
>> With another line.

---

$$
\delta = 2 G \frac{M_\oplus}{R_\oplus}\hat{R}
$$

```swift -- title="my_code_block.swift"
func myFunction() {
    return MyView.myExtensions()
}
```

Docs??

"#;
    render_test(test_content, "python-code-block").await;
}
