use crate::{
    inner_string_text, AnyJsImportClause, AnyJsNamedImportSpecifier, JsImport, JsImportNamedClause,
    JsModuleSource, JsSyntaxToken,
};
use biome_rowan::{AstNode, SyntaxResult, TokenText};

impl JsImport {
    /// It checks if the source of an import against the string `source_to_check`
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_js_syntax::T;
    ///
    /// let source = make::js_module_source(make::js_string_literal("react"));
    /// let binding = make::js_identifier_binding(make::ident("React"));
    /// let clause = make::js_import_default_clause(binding.into(), make::token(T![from]), source).build();
    /// let import = make::js_import(make::token(T![import]), clause.into()).build();
    ///
    /// assert_eq!(import.source_text().unwrap().text(), "react");
    /// ```
    pub fn source_text(&self) -> SyntaxResult<TokenText> {
        self.import_clause()?.source()?.inner_string_text()
    }
}

impl AnyJsImportClause {
    /// Source of this import clause.
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_js_syntax::T;
    ///
    /// let source = make::js_module_source(make::js_string_literal("react"));
    /// let binding = make::js_identifier_binding(make::ident("React"));
    /// let clause = make::js_import_default_clause(binding.into(), make::token(T![from]), source).build();
    ///
    /// assert_eq!(clause.source().unwrap().inner_string_text().unwrap().text(), "react");
    /// ```
    pub fn source(&self) -> SyntaxResult<JsModuleSource> {
        match self {
            Self::JsImportBareClause(node) => node.source(),
            Self::JsImportDefaultClause(node) => node.source(),
            Self::JsImportNamedClause(node) => node.source(),
            Self::JsImportNamespaceClause(node) => node.source(),
        }
    }
}

impl AnyJsNamedImportSpecifier {
    /// LOcal name of this import specifier
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_js_syntax::{AnyJsNamedImportSpecifier, T};
    ///
    /// let binding = make::js_identifier_binding(make::ident("React"));
    /// let specifier = make::js_shorthand_named_import_specifier(binding.into()).build();
    /// let specifier = AnyJsNamedImportSpecifier::JsShorthandNamedImportSpecifier(specifier);
    ///
    /// assert_eq!(specifier.local_name().unwrap().text_trimmed(), "React");
    /// ```
    pub fn local_name(&self) -> Option<JsSyntaxToken> {
        match self {
            AnyJsNamedImportSpecifier::JsNamedImportSpecifier(specifier) => {
                specifier.name().ok()?.value().ok()
            }
            AnyJsNamedImportSpecifier::JsShorthandNamedImportSpecifier(specifier) => specifier
                .local_name()
                .ok()?
                .as_js_identifier_binding()?
                .name_token()
                .ok(),
            AnyJsNamedImportSpecifier::JsBogusNamedImportSpecifier(_) => None,
        }
    }
}

impl AnyJsNamedImportSpecifier {
    /// Type token of the import specifier.
    ///
    /// ```ts
    /// import { type X }
    ///          ^^^^
    /// ```
    pub fn type_token(&self) -> Option<JsSyntaxToken> {
        match self {
            Self::JsBogusNamedImportSpecifier(_) => None,
            Self::JsNamedImportSpecifier(specifier) => specifier.type_token(),
            Self::JsShorthandNamedImportSpecifier(specifier) => specifier.type_token(),
        }
    }

    /// Returns the import clause that includes this specifier.
    pub fn import_named_clause(&self) -> Option<JsImportNamedClause> {
        JsImportNamedClause::cast(self.syntax().ancestors().nth(3)?)
    }

    /// Returns `true` if this specifier or its import clause has **only** a type modifier.
    pub fn imports_only_types(&self) -> bool {
        self.type_token().is_some()
            || self
                .import_named_clause()
                .and_then(|x| x.type_token())
                .is_some()
    }
}

impl JsModuleSource {
    /// Get the inner text of a string not including the quotes
    /// ## Examples
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_rowan::TriviaPieceKind;
    ///
    /// let source_token = make::js_string_literal("react")
    ///     .with_leading_trivia(vec![(TriviaPieceKind::Whitespace, " ")]);
    /// let source = make::js_module_source(source_token);
    ///
    /// assert_eq!(source.inner_string_text().unwrap().text(), "react");
    /// ```
    pub fn inner_string_text(&self) -> SyntaxResult<TokenText> {
        Ok(inner_string_text(&self.value_token()?))
    }
}
