use askama::Template;

/// ## Template (HTML)
///
/// ```askama
/// <div class="w-full grid grid-cols-[1fr_auto_1fr] gap-x-4 place-items-center my-6">
/// <div class="bg-border h-[2px] w-full"></div>
/// <div class="text-muted-foreground text-sm my-0! h-fit text-center [&>*]:text-center [&>*]:text-muted-foreground! [&>p]:text-sm [&>*]:my-0! [&>*]:h-fit! max-w-[min(250px,50vw)]">{{children | safe}}</div>
/// <div class="bg-border h-[2px] w-full"></div>
/// </div>
/// ```
#[derive(Template)]
#[template(ext = "html", in_doc = true)]
pub struct HrWithChildrenHTMLTemplate {
    pub children: String,
}
