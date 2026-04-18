use askama::Template;

/// Accepts a tuple with `(Content, Id)`
#[derive(Template)]
#[template(ext = "html")]
pub enum HeadingHtmlTemplate {
    #[template(ext = "html",
               source = "<h1 id=\"{{self.1}}\" class=\"block scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl text-foreground\">{{self.0}}</h1>")]
    H1(String, String),
    #[template(ext = "html",
               source = "<h2 id=\"{{self.1}}\" class=\"block scroll-m-20 pb-2 text-3xl font-semibold tracking-tight first:mt-0\">{{self.0}}</h2>")]
    H2(String, String),
    #[template(ext = "html",
               source = "<h3 id=\"{{self.1}}\" class=\"block scroll-m-20 text-2xl font-semibold tracking-tight\">{{self.0}}</h3>")]
    H3(String, String),
    #[template(ext = "html",
               source = "<h4 id=\"{{self.1}}\" class\"block scroll-m-20 text-xl font-semibold tracking-tight text-foreground\">{{self.0}}</h4>")]
    H4(String, String),
    #[template(ext = "html",
               source = "<h5 id=\"{{self.1}}\" class=\"block scroll-m-20 text-lg font-semibold tracking-tight text-foreground\">{{self.0}}</h5>")]
    H5(String, String),
    #[template(ext = "html",
               source = "<h6 id=\"{{self.1}}\" class\"block scroll-m-20 text-lg tracking-tight text-foreground\">{{self.0}}</h6>")]
    H6(String, String),
}
