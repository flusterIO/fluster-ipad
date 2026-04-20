import fs, { globSync } from "fs";
import path from "path";

const dir = path.resolve(__dirname, "../node_modules/katex/dist/fonts/");

const formatName = (s: string): string => {
    return path
        .basename(s)
        .replaceAll("-", "_")
        .replace(".woff2", "")
        .toLowerCase();
};

const x = globSync(path.join(dir, "*.woff2")).map((t) => {
    return {
        input: t,
        name: formatName(t),
        output: path.join(
            process.env.FLUSTER_IOS_ROOT,
            `packages/rust/conundrum/src/output/html/glue/component_glue_output/other/fonts/${path.basename(t)}`,
        ),
        outputAssetPath: path.join(
            process.env.FLUSTER_IOS_ROOT,
            `packages/rust/conundrum/src/output/html/glue/individual_glue_assets/${formatName(path.basename(t).replace(".wmff2", ""))}.rs`,
        ),
    };
});

for (const p of x) {
    fs.copyFileSync(p.input, p.output);
    const content = `use crate::output::html::glue::{
    component_glue_manager::{AnyGlueCodeKey, JS_GLUE_CODE_MAP, WebGlueCodeGeneralFiles},
    glue_asset::GlueCssAsset,
};

pub struct KatexFontAsset(WebGlueCodeGeneralFiles);

impl Default for KatexFontAsset {
    fn default() -> Self {
        Self(WebGlueCodeGeneralFiles::${p.name[0].toUpperCase()}${p.name.slice(1)})
    }
}

impl GlueCssAsset for KatexFontAsset {
    fn append_to_css(&self, content: &mut String, _: &bool) {
        if let Some(css_content) = JS_GLUE_CODE_MAP::get_by_key(AnyGlueCodeKey::General(self.0.clone())) {
            *content += css_content.as_str();
        } else {
            eprintln!("Could not find Katex css!!!");
        }
    }
}
`;
    fs.writeFileSync(p.outputAssetPath, content, { encoding: "utf-8" });
}

console.log(
    "x: ",
    x.map((n) => n.name),
);

let enumString = `
use serde::{Deserialize, Serialize};
#[derive(strum_macros::Display, Serialize, Deserialize, Clone)]
pub enum WebGlueCodeGeneralFiles {
    #[serde(rename = "styles.css")]
    #[strum(to_string = "styles.css")]
    Styles,
    #[serde(rename = "katex.min.css")]
    #[strum(to_string = "katex.min.css")]
    KatexCss,
    #[serde(rename = "katex.min.css")]
    #[strum(to_string = "katex.min.css")]
    KatexFont,
`;

for (const k of x) {
    enumString += `#[serde(rename = "${k.name}.woff2")]
#[strum(to_string = "${k.name}.woff2")]
#[allow(non_camel_case_types)]
${k.name[0].toUpperCase()}${k.name.slice(1)},
`;
}

fs.writeFileSync(
    path.join(
        process.env.FLUSTER_IOS_ROOT,
        "packages/rust/conundrum/src/output/html/glue/webglue_general_files.rs",
    ),
    `${enumString}}`,
    { encoding: "utf-8" },
);
