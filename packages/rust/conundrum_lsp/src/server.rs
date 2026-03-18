use conundrum::lang::runtime::parse_conundrum_string::parse_conundrum_string;
use conundrum::lang::runtime::run_conundrum::{ParseMdxOptions, run_conundrum};
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

struct Backend {
    client: Client,
}

// #[tower_lsp::async_trait]
// impl LanguageServer for Backend {
//     async fn initialize(&self, _: InitializeParams) ->
// Result<InitializeResult> {         Ok(InitializeResult {
//             capabilities: ServerCapabilities {
//                 text_document_sync: Some(TextDocumentSyncCapability::Kind(
//                     TextDocumentSyncKind::FULL,
//                 )),
//                 ..Default::default()
//             },
//             ..Default::default()
//         })
//     }

//     async fn did_open(&self, params: DidOpenTextDocumentParams) {
//         self.validate_document(params.text_document.uri,
// params.text_document.text)             .await;
//     }

//     async fn did_change(&self, params: DidChangeTextDocumentParams) {
//         // In a real Fluster app, you'd handle incremental sync,
//         // but for now, we take the full text.
//         if let Some(event) = params.content_changes.first() {
//             self.validate_document(params.text_document.uri,
// event.text.clone())                 .await;
//         }
//     }

//     async fn shutdown(&self) -> Result<()> {
//         Ok(())
//     }
// }

// impl Backend {
//     async fn validate_document(&self, uri: Url, text: String) {
//         let mut diagnostics = Vec::new();

//         let mut text_clone = text.as_str();

//         if let Err(e) = parse_conundrum_string(&mut text_clone) {
//             // Convert NOM error to LSP Diagnostic
//             if let nom::Err::Error(err) = e {
//                 let (line, col) = (err.input.location_line(),
//
// err.input.get_column());

//                 diagnostics.push(Diagnostic {
//                     range: Range {
//                         start: Position::new(line - 1, (col - 1) as u3
// 2),                         end: Position::new(line - 1, (col) as u32), //
// Highlight the specific char                     },
//                     severity: Some(DiagnosticSeverity::ERROR),
//                     message: "Invalid Fluster syntax".to_string(),
//                     ..Default::default()
//                 });
//             }
//         }

//         self.client
//             .publish_diagnostics(uri, diagnostics, None)
//             .await;
//     }
// }

// #[tokio::main]
// async fn main() {
//     let stdin = tokio::io::stdin();
//     let stdout = tokio::io::stdout();

//     let (service, socket) = LspService::new(|client| Backend { client });
//     Server::new(stdin, stdout, socket).serve(service).await;
// }
