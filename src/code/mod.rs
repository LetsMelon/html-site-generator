use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};

use inkjet::formatter::{Theme, ThemedHtml};
use inkjet::{Highlighter, Language};

use crate::html::{IntoHtmlNode, IntoHtmlNodeResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CodeBlockLanguage {
    Rust,
}

impl Into<Language> for CodeBlockLanguage {
    fn into(self) -> Language {
        match self {
            CodeBlockLanguage::Rust => Language::Rust,
        }
    }
}

#[derive(Debug)]
pub struct CodeBlock<'a> {
    language: CodeBlockLanguage,
    source_code: &'a str,
}

impl<'a> CodeBlock<'a> {
    pub fn new(language: CodeBlockLanguage, source_code: &'a str) -> Self {
        CodeBlock {
            language,
            source_code,
        }
    }
}

impl IntoHtmlNode for CodeBlock<'_> {
    fn transform_into_raw_html(&self, buffer: &mut dyn Write) -> IntoHtmlNodeResult<()> {
        let theme = ThemedHtml::new(Theme::from_str(include_str!("../../theme.toml"))?);
        let mut highlighter = Highlighter::new();

        // TODO maybe change signature of `transform_into_raw_*` to use generics, and not dyn
        let mut boxed_writer = Box::new(buffer);

        writeln!(boxed_writer, "<div class=\"code-block\">")?;
        highlighter.highlight_to_writer(
            self.language.into(),
            &theme,
            self.source_code,
            boxed_writer.as_mut(),
        )?;
        writeln!(boxed_writer, "</div>")?;

        Ok(())
    }

    fn transform_into_raw_css(&self, buffer: &mut dyn Write) -> IntoHtmlNodeResult<()> {
        static HAS_ALREADY_TRANSFORMED: AtomicBool = AtomicBool::new(false);

        if HAS_ALREADY_TRANSFORMED
            .compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed)
            .is_ok()
        {
            writeln!(buffer, ".code-block {{")?;
            for (key, value) in vec![
                ("width", "40rem"),
                ("overflow", "hidden"),
                ("border-radius", "10px"),
                ("padding", "7px"),
                ("margin", "5px"),
                ("background-color", "var(--bg0)"),
            ] {
                buffer
                    .write_fmt(format_args!("{}: {};", key, value))
                    .unwrap();
            }
            writeln!(buffer, "}}")?;
            writeln!(
                buffer,
                ".code-block pre {{ margin: 0px; color: var(--fg); }}"
            )?;

            // TODO what happens if two 'components' add values to ':root'
            writeln!(buffer, ":root {{").unwrap();
            for (key, value) in vec![
                //  copied from https://github.com/CptPotato/helix-themes/blob/main/palettes/edge/neon
                ("black", "#202023"),
                ("bg0", "#2b2d3a"),
                ("bg1", "#333648"),
                ("bg2", "#363a4e"),
                ("bg3", "#393e53"),
                ("bg4", "#3f445b"),
                ("bg_grey", "#7a819d"),
                ("bg_red", "#ec7279"),
                ("diff_red", "#55393d"),
                ("bg_green", "#a0c980"),
                ("diff_green", "#394634"),
                ("bg_blue", "#6cb6eb"),
                ("diff_blue", "#354157"),
                ("bg_purple", "#d38aea"),
                ("diff_yellow", "#4e432f"),
                ("fg", "#c5cdd9"),
                ("red", "#ec7279"),
                ("orange", "#e59b77"),
                ("yellow", "#deb974"),
                ("green", "#a0c980"),
                ("cyan", "#5dbbc1"),
                ("blue", "#6cb6eb"),
                ("purple", "#d38aea"),
                ("grey", "#7e8294"),
                ("grey_dim", "#5c6071"),
            ] {
                buffer.write_fmt(format_args!("--{}: {};", key, value))?;
            }
            writeln!(buffer, "}}")?;
        }

        Ok(())
    }

    fn transform_into_raw_js(&self, _buffer: &mut dyn Write) -> IntoHtmlNodeResult<()> {
        Ok(())
    }
}
