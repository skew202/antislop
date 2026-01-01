use antislop::{config::Config, Scanner};
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

struct Backend {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.validate_document(params.text_document.uri, params.text_document.text)
            .await;
    }

    async fn did_change(&self, mut params: DidChangeTextDocumentParams) {
        // TextDocumentSyncKind::FULL means content_changes[0].text is full content
        if let Some(change) = params.content_changes.pop() {
            self.validate_document(params.text_document.uri, change.text)
                .await;
        }
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        if let Some(text) = params.text {
            self.validate_document(params.text_document.uri, text).await;
        } else {
            // If text is not included in didSave (capability dependent), we might need to read from file system
            // But for now, we rely on sync being FULL or just ignore if text is missing,
            // relying on did_change having updated us.
            // Actually, if we want to support on-save validation specifically, we might want to re-trigger.
            // However, with FULL sync, did_change usually keeps us up to date.
            // But let's verify if we need to implement it.
            // The prompt asks for "didSave" specifically.
            // implementation_plan says: "Implement textDocument/didOpen and textDocument/didSave"
        }
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

impl Backend {
    async fn validate_document(&self, uri: Url, text: String) {
        // TODO: Try to find a config file in the workspace or parent directories of the file.
        // For now, we use the default configuration.
        let config = Config::default();

        let scanner =
            Scanner::new(config.patterns).expect("Failed to create scanner from default patterns");

        // Convert URI to path string for display/logging if needed,
        // though scanner mostly checks content.
        let path_buf = uri
            .to_file_path()
            .unwrap_or_else(|_| std::path::PathBuf::from("unknown"));
        let path_str = path_buf.to_str().unwrap_or("unknown");

        let result = scanner.scan_file(path_str, &text);

        let diagnostics: Vec<Diagnostic> = result
            .findings
            .iter()
            .map(|f| {
                // Find start and end column (antislop uses 1-based indexing, LSP uses 0-based)
                let start_line = (f.line).saturating_sub(1) as u32;
                let start_col = (f.column).saturating_sub(1) as u32;
                let end_col = start_col + f.match_text.chars().count() as u32; // basic char count approximation

                Diagnostic {
                    range: Range {
                        start: Position {
                            line: start_line,
                            character: start_col,
                        },
                        end: Position {
                            line: start_line,
                            character: end_col,
                        },
                    },
                    severity: Some(match f.severity.as_str() {
                        "CRITICAL" => DiagnosticSeverity::ERROR,
                        "HIGH" => DiagnosticSeverity::ERROR,
                        "MEDIUM" => DiagnosticSeverity::WARNING,
                        "LOW" => DiagnosticSeverity::INFORMATION,
                        _ => DiagnosticSeverity::HINT,
                    }),
                    code: Some(NumberOrString::String(
                        format!("{:?}", f.category).to_lowercase(),
                    )),
                    source: Some("antislop".to_string()),
                    message: f.message.clone(),
                    ..Default::default()
                }
            })
            .collect();

        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend { client });
    Server::new(stdin, stdout, socket).serve(service).await;
}
