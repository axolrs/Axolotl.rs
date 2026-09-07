// Owner: PascalElixir / axolrs (GitHub org)
// File: Server - the Gills tower-lsp backend: full LSP lifecycle over Content-Length framed stdio with document sync, diagnostics and language features.

use std::collections::HashMap;

use tokio::io::{AsyncRead, AsyncWrite};
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use crate::analysis::{self, Analysis};
use crate::line_index::LineIndex;
use crate::{SERVER_NAME, SERVER_VERSION};

/// The Gills language server backend served by tower-lsp.
pub struct Backend {
    client: Client,
    docs: RwLock<HashMap<Url, String>>,
}

impl Backend {
    /// Construct a backend around a tower-lsp client handle.
    pub fn new(client: Client) -> Backend {
        Backend { client, docs: RwLock::new(HashMap::new()) }
    }

    /// Store the document text and publish fresh diagnostics for it.
    async fn store_and_publish(&self, uri: &Url, text: String, version: Option<i32>) {
        let diags = {
            let mut docs = self.docs.write().await;
            docs.insert(uri.clone(), text.clone());
            analysis::analyze(&text).lsp_diagnostics()
        };
        self.client.publish_diagnostics(uri.clone(), diags, version).await;
    }

    /// Analyze the stored document at the given URI, if open.
    async fn analysis_of(&self, uri: &Url) -> Option<Analysis> {
        let text = self.docs.read().await.get(uri).cloned()?;
        Some(analysis::analyze(&text))
    }

    /// Copy the open-document set for cross-document queries.
    async fn snapshot(&self) -> Vec<(Url, String)> {
        self.docs
            .read()
            .await
            .iter()
            .map(|(uri, text)| (uri.clone(), text.clone()))
            .collect()
    }

    /// Convert a cursor position in an open document into a byte offset.
    async fn offset_at(&self, uri: &Url, position: Position) -> Option<u32> {
        let text = self.docs.read().await.get(uri).cloned()?;
        let index = LineIndex::new(&text);
        Some(index.offset_of(&text, position))
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    /// Advertise the Gills feature set for the `initialize` handshake.
    async fn initialize(&self, _params: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                position_encoding: Some(PositionEncodingKind::UTF16),
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(vec![
                        ".".to_string(),
                        ":".to_string(),
                        "@".to_string(),
                    ]),
                    resolve_provider: Some(false),
                    ..Default::default()
                }),
                definition_provider: Some(OneOf::Left(true)),
                references_provider: Some(OneOf::Left(true)),
                document_symbol_provider: Some(OneOf::Left(true)),
                rename_provider: Some(OneOf::Left(true)),
                workspace_symbol_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: SERVER_NAME.to_string(),
                version: Some(SERVER_VERSION.to_string()),
            }),
        })
    }

    /// Acknowledge initialization; Gills needs no client interaction here.
    async fn initialized(&self, _params: InitializedParams) {}

    /// Acknowledge the shutdown request; tower-lsp ends the loop on `exit`.
    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    /// Store the opened document and publish its diagnostics.
    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let doc = params.text_document;
        self.store_and_publish(&doc.uri, doc.text, Some(doc.version)).await;
    }

    /// Apply full-document (and, defensively, ranged) changes, then publish diagnostics.
    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        let version = params.text_document.version;
        let mut text = self.docs.read().await.get(&uri).cloned().unwrap_or_default();
        for change in &params.content_changes {
            match &change.range {
                None => text = change.text.clone(),
                Some(range) => {
                    let index = LineIndex::new(&text);
                    let start = index.offset_of(&text, range.start) as usize;
                    let end = index.offset_of(&text, range.end) as usize;
                    if start <= end && end <= text.len() {
                        text.replace_range(start..end, &change.text);
                    }
                }
            }
        }
        self.store_and_publish(&uri, text, Some(version)).await;
    }

    /// Refresh diagnostics on save, adopting the saved text when the client sends it.
    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        let uri = params.text_document.uri;
        match params.text {
            Some(text) => self.store_and_publish(&uri, text, None).await,
            None => {
                let diags = self
                    .analysis_of(&uri)
                    .await
                    .map(|a| a.lsp_diagnostics())
                    .unwrap_or_default();
                self.client.publish_diagnostics(uri, diags, None).await;
            }
        }
    }

    /// Drop the closed document and clear its published diagnostics.
    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;
        self.docs.write().await.remove(&uri);
        self.client.publish_diagnostics(uri, Vec::new(), None).await;
    }

    /// Answer hover with the signature and ownership mode of the identifier at the cursor.
    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let td = params.text_document_position_params;
        let uri = td.text_document.uri;
        let offset = self.offset_at(&uri, td.position).await;
        let hover = match (self.analysis_of(&uri).await, offset) {
            (Some(a), Some(offset)) => a.hover(offset),
            _ => None,
        };
        Ok(hover)
    }

    /// Answer completion with Axolotl keywords plus the document's own items.
    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let td = params.text_document_position;
        let uri = td.text_document.uri;
        let offset = self.offset_at(&uri, td.position).await;
        let items = match (self.analysis_of(&uri).await, offset) {
            (Some(a), Some(offset)) => a.completions(offset),
            _ => Vec::new(),
        };
        Ok(Some(CompletionResponse::Array(items)))
    }

    /// Answer goto definition with the declaring identifier of the item under the cursor.
    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let td = params.text_document_position_params;
        let uri = td.text_document.uri;
        let offset = self.offset_at(&uri, td.position).await;
        let location = match offset {
            Some(offset) => {
                let docs = self.snapshot().await;
                analysis::definition_in_docs(&docs, &uri, offset)
            }
            None => None,
        };
        Ok(location.map(GotoDefinitionResponse::Scalar))
    }

    /// Answer find references with the occurrences of the resolved binding across open documents.
    async fn references(&self, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        let td = params.text_document_position;
        let uri = td.text_document.uri;
        let offset = self.offset_at(&uri, td.position).await;
        let locations = match offset {
            Some(offset) => {
                let docs = self.snapshot().await;
                analysis::references_in_docs(
                    &docs,
                    &uri,
                    offset,
                    params.context.include_declaration,
                )
            }
            None => None,
        };
        Ok(locations)
    }

    /// Answer document symbols with the nested top-level outline.
    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let uri = params.text_document.uri;
        let symbols = self
            .analysis_of(&uri)
            .await
            .map(|a| a.document_symbols())
            .unwrap_or_default();
        Ok(Some(DocumentSymbolResponse::Nested(symbols)))
    }

    /// Answer rename with edits for the resolved binding across open documents.
    async fn rename(&self, params: RenameParams) -> Result<Option<WorkspaceEdit>> {
        let td = params.text_document_position;
        let uri = td.text_document.uri;
        let offset = self.offset_at(&uri, td.position).await;
        let edit = match offset {
            Some(offset) => {
                let docs = self.snapshot().await;
                analysis::rename_in_docs(&docs, &uri, offset, &params.new_name)
            }
            None => None,
        };
        Ok(edit)
    }

    /// Answer workspace symbols with top-level declarations whose name contains the query.
    #[allow(deprecated)]
    async fn symbol(&self, params: WorkspaceSymbolParams) -> Result<Option<Vec<SymbolInformation>>> {
        let query = params.query.to_lowercase();
        let mut symbols: Vec<SymbolInformation> = Vec::new();
        for (uri, text) in self.snapshot().await {
            let a = analysis::analyze(&text);
            for decl in a.item_decls() {
                if !query.is_empty() && !decl.name.to_lowercase().contains(&query) {
                    continue;
                }
                symbols.push(SymbolInformation {
                    name: decl.name.clone(),
                    kind: decl_symbol_kind(decl.kind),
                    location: Location {
                        uri: uri.clone(),
                        range: a.index.range_of(
                            &a.src,
                            decl.ident_span.start,
                            decl.ident_span.end,
                        ),
                    },
                    tags: None,
                    deprecated: None,
                    container_name: None,
                });
            }
        }
        Ok(Some(symbols))
    }
}

/// Map a declaration kind onto the flat workspace-symbol kind.
fn decl_symbol_kind(kind: analysis::DeclKind) -> SymbolKind {
    match kind {
        analysis::DeclKind::Fn => SymbolKind::FUNCTION,
        analysis::DeclKind::Method => SymbolKind::METHOD,
        analysis::DeclKind::Struct => SymbolKind::STRUCT,
        analysis::DeclKind::Enum => SymbolKind::ENUM,
        analysis::DeclKind::Interface => SymbolKind::INTERFACE,
        analysis::DeclKind::Const => SymbolKind::CONSTANT,
    }
}

/// Serve the Gills LSP over framed stdio until the client disconnects or exits.
pub async fn run_stdio() -> std::io::Result<()> {
    serve_transport(tokio::io::stdin(), tokio::io::stdout()).await
}

/// Serve the Gills LSP over any byte transport (split out for reuse and testing).
pub async fn serve_transport<I, O>(stdin: I, stdout: O) -> std::io::Result<()>
where
    I: AsyncRead + Unpin,
    O: AsyncWrite,
{
    let (service, socket) = LspService::new(Backend::new);
    Server::new(stdin, stdout, socket).serve(service).await;
    Ok(())
}
