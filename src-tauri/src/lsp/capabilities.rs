use lsp_types::{
  ChangeAnnotationWorkspaceEditClientCapabilities, ClientCapabilities, CodeActionCapabilityResolveSupport, CodeActionClientCapabilities, CodeActionKindLiteralSupport, CodeActionLiteralSupport, CodeLensClientCapabilities, CodeLensWorkspaceClientCapabilities, CompletionClientCapabilities, CompletionItemKind, CompletionItemKindCapability, CompletionListCapability, DiagnosticClientCapabilities, DiagnosticTag, DiagnosticWorkspaceClientCapabilities, DidChangeWatchedFilesClientCapabilities, DocumentFormattingClientCapabilities, DocumentHighlightClientCapabilities, DocumentLinkClientCapabilities, DocumentSymbolClientCapabilities, DynamicRegistrationClientCapabilities, FoldingRangeCapability, FoldingRangeClientCapabilities, FoldingRangeKind, FoldingRangeKindCapability, GeneralClientCapabilities, GotoCapability, HoverClientCapabilities, InlayHintClientCapabilities, InlayHintResolveClientCapabilities, InlayHintWorkspaceClientCapabilities, InlineValueWorkspaceClientCapabilities, InsertTextMode, MarkdownClientCapabilities, MarkupKind, MessageActionItemCapabilities, PositionEncodingKind, PrepareSupportDefaultBehavior, PublishDiagnosticsClientCapabilities, ReferenceClientCapabilities, RegularExpressionsClientCapabilities, RenameClientCapabilities, ResourceOperationKind, SelectionRangeClientCapabilities, SemanticTokenModifier, SemanticTokenType, SemanticTokensClientCapabilities, SemanticTokensClientCapabilitiesRequests, SemanticTokensFullOptions, SemanticTokensWorkspaceClientCapabilities, ShowDocumentClientCapabilities, ShowMessageRequestClientCapabilities, SignatureHelpClientCapabilities, SignatureInformationSettings, StaleRequestSupportClientCapabilities, SymbolKind, SymbolKindCapability, SymbolTag, TagSupport, TextDocumentClientCapabilities, TextDocumentSyncClientCapabilities, TokenFormat, WindowClientCapabilities, WorkspaceClientCapabilities, WorkspaceEditClientCapabilities, WorkspaceFileOperationsClientCapabilities, WorkspaceSymbolClientCapabilities, WorkspaceSymbolResolveSupportCapability
};

pub(crate) fn get_capabilities() -> ClientCapabilities {
  let symbol_kind = Some(SymbolKindCapability {
    value_set: Some(vec![
      SymbolKind::FILE,
      SymbolKind::MODULE,
      SymbolKind::NAMESPACE,
      SymbolKind::PACKAGE,
      SymbolKind::CLASS,
      SymbolKind::METHOD,
      SymbolKind::PROPERTY,
      SymbolKind::FIELD,
      SymbolKind::CONSTRUCTOR,
      SymbolKind::ENUM,
      SymbolKind::INTERFACE,
      SymbolKind::FUNCTION,
      SymbolKind::VARIABLE,
      SymbolKind::CONSTANT,
      SymbolKind::STRING,
      SymbolKind::NUMBER,
      SymbolKind::BOOLEAN,
      SymbolKind::ARRAY,
      SymbolKind::OBJECT,
      SymbolKind::KEY,
      SymbolKind::NULL,
      SymbolKind::ENUM_MEMBER,
      SymbolKind::STRUCT,
      SymbolKind::EVENT,
      SymbolKind::OPERATOR,
      SymbolKind::TYPE_PARAMETER,
    ]),
  });

  let tag_support = Some(TagSupport {
    value_set: vec![SymbolTag::DEPRECATED],
  });

  ClientCapabilities {
    text_document: Some(TextDocumentClientCapabilities {
      synchronization: Some(TextDocumentSyncClientCapabilities {
        dynamic_registration: Some(false),
        did_save: Some(true),
        will_save: Some(true),
        will_save_wait_until: Some(true),
      }),
      completion: Some(CompletionClientCapabilities {
        completion_item: Some(lsp_types::CompletionItemCapability {
          snippet_support: Some(true),
          ..Default::default()
        }),
        dynamic_registration: Some(false),
        completion_item_kind: Some(CompletionItemKindCapability {
          value_set: Some(vec![
            CompletionItemKind::TEXT,
            CompletionItemKind::METHOD,
            CompletionItemKind::FUNCTION,
            CompletionItemKind::CONSTRUCTOR,
            CompletionItemKind::FIELD,
            CompletionItemKind::VARIABLE,
            CompletionItemKind::CLASS,
            CompletionItemKind::INTERFACE,
            CompletionItemKind::MODULE,
            CompletionItemKind::PROPERTY,
            CompletionItemKind::UNIT,
            CompletionItemKind::VALUE,
            CompletionItemKind::ENUM,
            CompletionItemKind::KEYWORD,
            CompletionItemKind::SNIPPET,
            CompletionItemKind::COLOR,
            CompletionItemKind::FILE,
            CompletionItemKind::REFERENCE,
            CompletionItemKind::FOLDER,
            CompletionItemKind::ENUM_MEMBER,
            CompletionItemKind::CONSTANT,
            CompletionItemKind::STRUCT,
            CompletionItemKind::EVENT,
            CompletionItemKind::OPERATOR,
            CompletionItemKind::TYPE_PARAMETER,
          ]),
        }),
        context_support: Some(true),
        insert_text_mode: Some(InsertTextMode::ADJUST_INDENTATION),
        completion_list: Some(CompletionListCapability {
          item_defaults: None,
        }),
      }),
      hover: Some(HoverClientCapabilities {
        dynamic_registration: Some(false),
        content_format: Some(vec![MarkupKind::PlainText, MarkupKind::Markdown]),
      }),
      signature_help: Some(SignatureHelpClientCapabilities {
        dynamic_registration: Some(false),
        signature_information: Some(SignatureInformationSettings {
          documentation_format: Some(vec![MarkupKind::PlainText, MarkupKind::Markdown]),
          parameter_information: Some(lsp_types::ParameterInformationSettings {
            label_offset_support: Some(true),
          }),
          active_parameter_support: Some(true),
        }),
        context_support: Some(true),
      }),
      references: Some(ReferenceClientCapabilities {
        dynamic_registration: Some(false),
      }),
      document_highlight: Some(DocumentHighlightClientCapabilities {
        dynamic_registration: Some(false),
      }),
      document_link: Some(DocumentLinkClientCapabilities {
        dynamic_registration: Some(false),
        tooltip_support: Some(true),
      }),
      document_symbol: Some(DocumentSymbolClientCapabilities {
        dynamic_registration: Some(false),
        symbol_kind: symbol_kind.clone(),
        hierarchical_document_symbol_support: Some(true),
        tag_support: tag_support.clone(),
      }),
      formatting: Some(DocumentFormattingClientCapabilities {
        dynamic_registration: Some(false),
      }),
      folding_range: Some(FoldingRangeClientCapabilities {
        dynamic_registration: Some(false),
        range_limit: Some(5000),
        line_folding_only: Some(true),
        folding_range: Some(FoldingRangeCapability {
          collapsed_text: Some(true),
        }),
        folding_range_kind: Some(FoldingRangeKindCapability {
          value_set: Some(vec![
            FoldingRangeKind::Comment,
            FoldingRangeKind::Imports,
            FoldingRangeKind::Region,
          ]),
        }),
      }),
      range_formatting: Some(DocumentFormattingClientCapabilities {
        dynamic_registration: Some(false),
      }),
      on_type_formatting: Some(DocumentFormattingClientCapabilities {
        dynamic_registration: Some(false),
      }),
      declaration: Some(GotoCapability {
        dynamic_registration: Some(false),
        link_support: Some(true),
      }),
      definition: Some(GotoCapability {
        dynamic_registration: Some(false),
        link_support: Some(true),
      }),
      type_definition: Some(GotoCapability {
        dynamic_registration: Some(false),
        link_support: Some(true),
      }),
      implementation: Some(GotoCapability {
        dynamic_registration: Some(false),
        link_support: Some(true),
      }),
      code_action: Some(CodeActionClientCapabilities {
        dynamic_registration: Some(false),
        is_preferred_support: Some(true),
        disabled_support: Some(true),
        data_support: Some(true),
        resolve_support: Some(CodeActionCapabilityResolveSupport {
          properties: vec!["edit".to_string()],
        }),
        code_action_literal_support: Some(CodeActionLiteralSupport {
          code_action_kind: CodeActionKindLiteralSupport {
            value_set: vec![
              "quickfix".to_string(),
              "refactor".to_string(),
              "refactor.extract".to_string(),
              "refactor.inline".to_string(),
              "refactor.rewrite".to_string(),
              "source".to_string(),
              "source.organizeImports".to_string(),
            ],
          },
        }),
        honors_change_annotations: Some(true),
      }),
      code_lens: Some(CodeLensClientCapabilities {
        dynamic_registration: Some(false),
      }),
      color_provider: Some(DynamicRegistrationClientCapabilities {
        dynamic_registration: Some(false),
      }),
      call_hierarchy: Some(DynamicRegistrationClientCapabilities {
        dynamic_registration: Some(false),
      }),
      rename: Some(RenameClientCapabilities {
        dynamic_registration: Some(false),
        prepare_support: Some(true),
        prepare_support_default_behavior: Some(PrepareSupportDefaultBehavior::IDENTIFIER),
        honors_change_annotations: Some(true),
      }),
      publish_diagnostics: Some(PublishDiagnosticsClientCapabilities {
        related_information: Some(true),
        tag_support: Some(TagSupport {
          value_set: vec![DiagnosticTag::DEPRECATED, DiagnosticTag::UNNECESSARY],
        }),
        version_support: Some(true),
        code_description_support: Some(true),
        data_support: Some(true),
      }),
      selection_range: Some(SelectionRangeClientCapabilities {
        dynamic_registration: Some(false),
      }),
      semantic_tokens: Some(SemanticTokensClientCapabilities {
        dynamic_registration: Some(false),
        requests: SemanticTokensClientCapabilitiesRequests {
          range: Some(true),
          full: Some(SemanticTokensFullOptions::Delta { delta: Some(true) }),
        },
        token_types: vec![
          SemanticTokenType::NAMESPACE,
          SemanticTokenType::TYPE,
          SemanticTokenType::CLASS,
          SemanticTokenType::ENUM,
          SemanticTokenType::INTERFACE,
          SemanticTokenType::STRUCT,
          SemanticTokenType::TYPE_PARAMETER,
          SemanticTokenType::PARAMETER,
          SemanticTokenType::VARIABLE,
          SemanticTokenType::PROPERTY,
          SemanticTokenType::ENUM_MEMBER,
          SemanticTokenType::EVENT,
          SemanticTokenType::FUNCTION,
          SemanticTokenType::METHOD,
          SemanticTokenType::MACRO,
          SemanticTokenType::KEYWORD,
          SemanticTokenType::MODIFIER,
          SemanticTokenType::COMMENT,
          SemanticTokenType::STRING,
          SemanticTokenType::NUMBER,
          SemanticTokenType::REGEXP,
          SemanticTokenType::OPERATOR,
          SemanticTokenType::DECORATOR,
        ],
        token_modifiers: vec![
          SemanticTokenModifier::DECLARATION,
          SemanticTokenModifier::DEFINITION,
          SemanticTokenModifier::READONLY,
          SemanticTokenModifier::STATIC,
          SemanticTokenModifier::DEPRECATED,
          SemanticTokenModifier::ABSTRACT,
          SemanticTokenModifier::ASYNC,
          SemanticTokenModifier::MODIFICATION,
          SemanticTokenModifier::DOCUMENTATION,
          SemanticTokenModifier::DEFAULT_LIBRARY,
        ],
        formats: vec![TokenFormat::RELATIVE],
        overlapping_token_support: Some(true),
        multiline_token_support: Some(true),
        server_cancel_support: Some(false),
        augments_syntax_tokens: None,
      }),
      linked_editing_range: Some(DynamicRegistrationClientCapabilities {
        dynamic_registration: Some(false),
      }),
      moniker: None,
      type_hierarchy: Some(DynamicRegistrationClientCapabilities {
        dynamic_registration: Some(false),
      }),
      inlay_hint: Some(InlayHintClientCapabilities {
        dynamic_registration: Some(false),
        resolve_support: Some(InlayHintResolveClientCapabilities { properties: vec![] }),
      }),
      inline_value: Some(DynamicRegistrationClientCapabilities {
        dynamic_registration: Some(false),
      }),
      diagnostic: Some(DiagnosticClientCapabilities {
        dynamic_registration: Some(false),
        related_document_support: Some(true),
      }),
    }),
    window: Some(WindowClientCapabilities {
      work_done_progress: Some(true),
      show_document: Some(ShowDocumentClientCapabilities { support: true }),
      show_message: Some(ShowMessageRequestClientCapabilities {
        message_action_item: Some(MessageActionItemCapabilities {
          additional_properties_support: Some(true),
        }),
      }),
    }),
    workspace: Some(WorkspaceClientCapabilities {
      apply_edit: Some(true),
      workspace_edit: Some(WorkspaceEditClientCapabilities {
        document_changes: Some(true),
        resource_operations: Some(vec![
          ResourceOperationKind::Create,
          ResourceOperationKind::Rename,
          ResourceOperationKind::Delete,
        ]),
        failure_handling: Some(lsp_types::FailureHandlingKind::Transactional),
        normalizes_line_endings: Some(true),
        change_annotation_support: Some(ChangeAnnotationWorkspaceEditClientCapabilities {
          groups_on_label: Some(true),
        }),
      }),
      did_change_configuration: Some(DynamicRegistrationClientCapabilities {
        dynamic_registration: Some(false),
      }),
      did_change_watched_files: Some(DidChangeWatchedFilesClientCapabilities {
        dynamic_registration: Some(false),
        relative_pattern_support: Some(true),
      }),
      symbol: Some(WorkspaceSymbolClientCapabilities {
        dynamic_registration: Some(false),
        symbol_kind,
        tag_support,
        resolve_support: Some(WorkspaceSymbolResolveSupportCapability {
          properties: vec!["location.range".to_string()],
        }),
      }),
      execute_command: Some(DynamicRegistrationClientCapabilities {
        dynamic_registration: Some(false),
      }),
      workspace_folders: Some(true),
      configuration: Some(true),
      semantic_tokens: Some(SemanticTokensWorkspaceClientCapabilities {
        refresh_support: Some(true),
      }),
      code_lens: Some(CodeLensWorkspaceClientCapabilities {
        refresh_support: Some(true),
      }),
      inlay_hint: Some(InlayHintWorkspaceClientCapabilities {
        refresh_support: Some(true),
      }),
      inline_value: Some(InlineValueWorkspaceClientCapabilities {
        refresh_support: Some(true),
      }),
      diagnostic: Some(DiagnosticWorkspaceClientCapabilities {
        refresh_support: Some(true),
      }),
      file_operations: Some(WorkspaceFileOperationsClientCapabilities {
        dynamic_registration: Some(false),
        did_create: Some(true),
        will_create: Some(true),
        did_rename: Some(true),
        will_rename: Some(true),
        did_delete: Some(true),
        will_delete: Some(true),
      }),
    }),
    general: Some(GeneralClientCapabilities {
      regular_expressions: Some(RegularExpressionsClientCapabilities {
        engine: "ECMAScript".to_string(),
        version: Some("ECMAScript".to_string()),
      }),
      markdown: Some(MarkdownClientCapabilities {
        parser: "marked".to_string(),
        version: Some("1.1.0".to_string()),
        allowed_tags: None,
      }),
      stale_request_support: Some(StaleRequestSupportClientCapabilities {
        cancel: true,
        retry_on_content_modified: vec![],
      }),
      position_encodings: Some(vec![PositionEncodingKind::UTF8]),
    }),
    experimental: None,
  }
}
