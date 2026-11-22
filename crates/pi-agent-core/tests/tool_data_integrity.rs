use pi_agent_core::tools::office::{DocxReader, TableData};
use pi_agent_core::tools::ToolRegistry;
use serde_json::{json, Value};
use std::process::Command;
use tempfile::tempdir;

#[test]
fn malformed_markdown_returns_errors_without_panicking() {
    // Boundaries: missing cells, bad separator, short/long rows, incomplete border.
    for markdown in [
        "|\n|",
        "| a |\n| not a separator |",
        "| a | b |\n| --- |",
        "| a | b |\n| --- | --- |\n| one |",
        "| a |\n| --- |\n| one | two |",
        "| a |\n| --- |\n| missing border",
    ] {
        assert!(
            TableData::parse_markdown(markdown).is_err(),
            "accepted {markdown:?}"
        );
    }
    let aligned = TableData::parse_markdown("| a | b |\n| :--- | ---: |\n| 1 | 2 |");
    assert_eq!(aligned.unwrap().rows, vec![vec!["1", "2"]]);
}

#[test]
fn csv_preserves_whitespace_and_empty_records() {
    // Boundaries: empty/header-only, quoted empty EOF, empty records, CR/LF/CRLF.
    assert!(TableData::parse_csv("", ',').is_err());
    let quoted_header = TableData::parse_csv("\"\"", ',').unwrap();
    assert_eq!(quoted_header.headers, vec![""]);
    assert!(quoted_header.rows.is_empty());
    for newline in ["\n", "\r", "\r\n"] {
        let csv = format!("name,value{newline}\" Alice \",  spaced  {newline},{newline}");
        let table = TableData::parse_csv(&csv, ',').unwrap();
        assert_eq!(
            table.rows,
            vec![vec![" Alice ", "  spaced  "], vec!["", ""]]
        );
        let roundtrip = TableData::parse_csv(&table.to_csv(','), ',').unwrap();
        assert_eq!(roundtrip.headers, table.headers);
        assert_eq!(roundtrip.rows, table.rows);
    }
    let empty_rows = TableData::parse_csv("header\n\n\"\"", ',').unwrap();
    assert_eq!(empty_rows.rows, vec![vec![""], vec![""]]);
}

#[test]
fn malformed_csv_quotes_are_rejected_without_removing_characters() {
    for csv in [
        "name\nab\"cd\"\n",
        "name\n\"ab\"cd\n",
        "name\n \"ab\"\n",
        "name\n\"ab\" \n",
    ] {
        assert!(TableData::parse_csv(csv, ',').is_err(), "accepted {csv:?}");
    }
    let table = TableData::parse_csv("name,note\n\"a\"\"b\",\"first\nsecond\"\n", ',').unwrap();
    assert_eq!(table.rows, vec![vec!["a\"b", "first\nsecond"]]);
}

#[test]
fn office_cli_csv_output_does_not_add_a_record() {
    let workspace = tempdir().unwrap();
    let table = TableData::new(
        vec!["name".into(), "note".into()],
        vec![
            vec!["Alice".into(), "first\nsecond".into()],
            vec!["".into(), "".into()],
        ],
    );
    let markdown = workspace.path().join("table.md");
    std::fs::write(&markdown, table.to_markdown()).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_office-cli"))
        .args(["table", "to-csv", "--file"])
        .arg(&markdown)
        .output()
        .unwrap();
    assert!(output.status.success());
    let csv = String::from_utf8(output.stdout).unwrap();
    assert_eq!(csv, table.to_csv(','));
    assert_eq!(TableData::parse_csv(&csv, ',').unwrap().rows, table.rows);
}

#[test]
fn markdown_requires_an_unescaped_closing_border() {
    for row in [r"| left\|", r"| left\\\|"] {
        assert!(TableData::parse_markdown(&format!("| a |\n| --- |\n{row}")).is_err());
    }
    let table = TableData::parse_markdown("| a |\n| --- |\n| left\\\\|").unwrap();
    assert_eq!(table.rows, vec![vec!["left\\"]]);
}

#[test]
fn markdown_roundtrip_preserves_cell_boundaries_and_text() {
    let cells = [
        "left|right",
        "first\nsecond",
        "first\r\nsecond",
        r"back\slash\|pipe",
        " literal <br> &amp; &#10; ",
        "\t tabs \t",
        "   ",
        "",
        "中文🙂",
    ];
    let table = TableData::new(
        vec!["header|with\nbreak".into(), " note ".into()],
        cells
            .iter()
            .map(|cell| vec![cell.to_string(), "other".into()])
            .collect(),
    );
    let markdown = table.to_markdown();
    assert_eq!(markdown.lines().count(), cells.len() + 2);
    let restored = TableData::parse_markdown(&markdown).unwrap();
    assert_eq!(restored.headers, table.headers);
    assert_eq!(restored.rows, table.rows);
    let escaped = TableData::parse_markdown("| a |\n| --- |\n| left\\|right |").unwrap();
    assert_eq!(escaped.rows, vec![vec!["left|right"]]);
}

#[test]
fn markdown_roundtrip_preserves_unicode_boundary_whitespace() {
    let table = TableData::new(
        vec!["\u{2003}name\u{a0}".into()],
        [
            "\u{2003}Alice\u{a0}",
            "\u{b}Bob\u{c}",
            "\u{3000}",
            "\t\n\r ",
        ]
        .iter()
        .map(|cell| vec![cell.to_string()])
        .collect(),
    );
    let restored = TableData::parse_markdown(&table.to_markdown()).unwrap();
    assert_eq!(restored.headers, table.headers);
    assert_eq!(restored.rows, table.rows);
}

#[tokio::test]
async fn explicit_search_read_errors_are_not_negative_results() {
    let workspace = tempdir().unwrap();
    std::fs::write(workspace.path().join("binary.dat"), [0xff, 0xfe]).unwrap();
    std::fs::write(workspace.path().join("text.txt"), "needle").unwrap();
    let grep = ToolRegistry::with_standard_tools(workspace.path())
        .get("grep")
        .unwrap();
    let error = grep
        .execute(json!({"pattern":"needle", "path":"binary.dat"}))
        .await
        .unwrap_err();
    assert!(format!("{error:#}").contains("binary.dat"));
    let nested = grep.execute(json!({"pattern":"needle"})).await.unwrap();
    assert_eq!(nested, "text.txt:1: needle");
}

#[tokio::test]
async fn invalid_search_paths_do_not_expand_to_the_workspace() {
    let workspace = tempdir().unwrap();
    std::fs::write(workspace.path().join("private.txt"), "needle").unwrap();
    let grep = ToolRegistry::with_standard_tools(workspace.path())
        .get("grep")
        .unwrap();
    for invalid in [json!(12), json!([]), json!({}), json!(false), Value::Null] {
        assert!(grep
            .execute(json!({"pattern":"needle", "path":invalid}))
            .await
            .is_err());
    }
    assert!(grep
        .execute(json!({"pattern":"needle"}))
        .await
        .unwrap()
        .contains("private.txt"));
}

#[tokio::test]
async fn tsv_default_delimiter_and_explicit_override_are_honored() {
    let workspace = tempdir().unwrap();
    let office = ToolRegistry::with_standard_tools(workspace.path())
        .get("office")
        .unwrap();
    for name in ["input.tsv", "input.TSV"] {
        std::fs::write(workspace.path().join(name), "name\tcount\nAlice\t7\n").unwrap();
        let markdown = office
            .execute(json!({"action":"csv_to_md", "file_path":name}))
            .await
            .unwrap();
        let table = TableData::parse_markdown(&markdown).unwrap();
        assert_eq!(table.headers, vec!["name", "count"]);
        assert_eq!(table.rows, vec![vec!["Alice", "7"]]);
        let stats = office
            .execute(json!({"action":"table_stats", "file_path":name, "target_col":"count"}))
            .await
            .unwrap();
        assert_eq!(serde_json::from_str::<Value>(&stats).unwrap()["sum"], 7.0);
    }
    std::fs::write(
        workspace.path().join("override.tsv"),
        "name;count\nAlice;7\n",
    )
    .unwrap();
    let markdown = office
        .execute(json!({"action":"csv_to_md", "file_path":"override.tsv", "delimiter":";"}))
        .await
        .unwrap();
    assert_eq!(
        TableData::parse_markdown(&markdown).unwrap().headers,
        vec!["name", "count"]
    );
}

#[tokio::test]
async fn invalid_delimiters_are_rejected_instead_of_truncated() {
    let workspace = tempdir().unwrap();
    let office = ToolRegistry::with_standard_tools(workspace.path())
        .get("office")
        .unwrap();
    for invalid in [
        json!(""),
        json!(",,"),
        json!("\""),
        json!("\n"),
        json!("\r"),
        json!(2),
        Value::Null,
    ] {
        assert!(office
            .execute(json!({"action":"csv_to_md", "content":"a,b\n1,2", "delimiter":invalid}))
            .await
            .is_err());
    }
}

#[tokio::test]
async fn invalid_slide_counts_are_rejected_by_tool_and_cli() {
    let workspace = tempdir().unwrap();
    let office = ToolRegistry::with_standard_tools(workspace.path())
        .get("office")
        .unwrap();
    // Boundaries: wrong types, below minimum, above maximum, omitted default.
    for invalid in [
        json!("4"),
        json!(-1),
        json!(3.5),
        json!(false),
        Value::Null,
        json!(0),
        json!(2),
        json!(101),
        json!(u64::MAX),
    ] {
        assert!(office
            .execute(json!({"action":"slides_gen", "num_slides":invalid}))
            .await
            .is_err());
    }
    for count in ["bad", "-1", "3.5", "0", "2", "101", "18446744073709551615"] {
        let output = Command::new(env!("CARGO_BIN_EXE_office-cli"))
            .args(["slides", "gen", "--slides", count])
            .output()
            .unwrap();
        assert!(!output.status.success(), "accepted {count}");
        assert!(output.stdout.is_empty(), "printed a default deck");
    }
    let missing = Command::new(env!("CARGO_BIN_EXE_office-cli"))
        .args(["slides", "gen", "--slides"])
        .output()
        .unwrap();
    assert!(!missing.status.success());
}

fn slide_count(markdown: &str) -> usize {
    markdown.lines().filter(|line| *line == "---").count() - 1
}

#[tokio::test]
async fn slide_counts_include_all_fixed_slides() {
    let workspace = tempdir().unwrap();
    let office = ToolRegistry::with_standard_tools(workspace.path())
        .get("office")
        .unwrap();
    for count in [3, 4, 5, 100] {
        let slides = office
            .execute(json!({"action":"slides_gen", "num_slides":count}))
            .await
            .unwrap();
        assert_eq!(slide_count(&slides), count);
        assert!(slides.contains("# 谢谢倾听 / Q&A"));
    }
    let default_slides = office
        .execute(json!({"action":"slides_gen"}))
        .await
        .unwrap();
    assert_eq!(slide_count(&default_slides), 5);
    let output = Command::new(env!("CARGO_BIN_EXE_office-cli"))
        .args(["slides", "gen", "--slides", "4"])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(slide_count(&String::from_utf8(output.stdout).unwrap()), 4);
}

#[test]
fn docx_entities_decode_once_and_stats_use_document_text() {
    let text =
        DocxReader::extract_text_from_bytes(include_bytes!("fixtures/xml-entities.docx")).unwrap();
    assert_eq!(text, "R&D <demo> \"quoted\" 'apostrophe' A 中 🙂 &amp;\n\n");
    let stats = DocxReader::compute_doc_stats(&text);
    assert_eq!(stats["characters_no_whitespace"], 37);
    assert_eq!(stats["words"], 8);
}
