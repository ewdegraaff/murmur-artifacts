//! `tool-echo`: returns the text it is given. A wasm32-wasip2 component exporting
//! `murmur:tool/run` (world `tool`).

#[cfg(target_arch = "wasm32")]
mod wasm_tool {
    wit_bindgen::generate!({
        path: "../../wit/guest",
        world: "tool",
        generate_all,
    });

    use exports::murmur::tool::run::{Guest, Status, ToolInput, ToolResult};

    struct Component;

    /// The `text` member of the call's JSON arguments, or `None` when it is not a string.
    fn text_of(data: &str) -> Option<String> {
        let key = data.find("\"text\"")?;
        let rest = data[key + 6..].trim_start().strip_prefix(':')?.trim_start();
        let rest = rest.strip_prefix('"')?;
        let mut out = String::new();
        let mut chars = rest.chars();
        while let Some(c) = chars.next() {
            match c {
                '"' => return Some(out),
                '\\' => match chars.next()? {
                    'n' => out.push('\n'),
                    't' => out.push('\t'),
                    other => out.push(other),
                },
                other => out.push(other),
            }
        }
        None
    }

    impl Guest for Component {
        fn run(input: ToolInput) -> ToolResult {
            let (status, summary) = match input.data.as_deref().and_then(text_of) {
                Some(text) => (Status::Passed, text),
                None => (Status::Error, "text must be a string".to_string()),
            };
            ToolResult {
                status,
                summary: Some(summary),
                data: None,
                data_path: None,
                truncated: false,
                metadata: Vec::new(),
                attachments: Vec::new(),
            }
        }
    }

    export!(Component);
}
