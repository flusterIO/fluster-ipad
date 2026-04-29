use askama::Template;

/// Accepts a tuple with `(Content, Subtitle, Id)`
#[derive(Template)]
#[template(ext = "html")]
pub enum HeadingSubtitleHtmlTemplate {
    #[template(ext = "html",
               source = "<div class=\"w-full flex flex-col justify-start items-center\"><h1 id=\"{{self.2.to_heading_id() | safe}}\" class=\"block w-full scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl text-foreground mb-0\">{{self.0 | safe}}</h1><div class=\"text-sm w-full text-muted-foreground not-prose mb-4\">{{self.1 | safe}}</div></div>")]
    H1(String, String, DOMId),
    #[template(ext = "html",
               source = "<div class=\"w-full flex flex-col justify-start items-center\"><h2 id=\"{{self.2.to_heading_id() | safe}}\" class=\"block w-full scroll-m-20 text-3xl font-semibold tracking-tight first:mt-0 mb-0\">{{self.0 | safe}}</h2><div class=\"text-sm w-full text-muted-foreground not-prose mb-4\">{{self.1 | safe}}</div></div>")]
    H2(String, String, DOMId),
    #[template(ext = "html",
               source = "<div class=\"w-full flex flex-col justify-start items-center\"><h3 id=\"{{self.2.to_heading_id() | safe}}\" class=\"block w-full scroll-m-20 text-2xl font-semibold tracking-tight mb-0\">{{self.0 | safe}}</h3><div class=\"text-sm w-full text-muted-foreground not-prose mb-4\">{{self.1 | safe}}</div></div>")]
    H3(String, String, DOMId),
    #[template(ext = "html",
               source = "<div class=\"w-full flex flex-col justify-start items-center\"><h4 id=\"{{self.2.to_heading_id() | safe}}\" class=\"block w-full scroll-m-20 text-xl font-semibold tracking-tight text-foreground mb-0\">{{self.0 | safe}}</h4><div class=\"text-sm w-full text-muted-foreground not-prose mb-4\">{{self.1 | safe}}</div></div>")]
    H4(String, String, DOMId),
    #[template(ext = "html",
               source = "<div class=\"w-full flex flex-col justify-start items-center\"><h5 id=\"{{self.2.to_heading_id() | safe}}\" class=\"block w-full scroll-m-20 text-lg font-semibold tracking-tight text-foreground mb-0\">{{self.0 | safe}}</h5><div class=\"text-sm w-full text-muted-foreground not-prose mb-4\">{{self.1 | safe}}</div></div>")]
    H5(String, String, DOMId),
    #[template(ext = "html",
               source = "<div class=\"w-full flex flex-col justify-start items-center\"><h6 id=\"{{self.2.to_heading_id() | safe}}\" class=\"block w-full scroll-m-20 text-lg tracking-tight text-foreground mb-0\">{{self.0 | safe}}</h6><div class=\"text-sm w-full text-muted-foreground not-prose mb-4\">{{self.1 | safe}}</div></div>")]
    H6(String, String, DOMId),
}
