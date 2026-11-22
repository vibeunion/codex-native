use pi_agent_core::tools::ToolRegistry;
use serde_json::{json, Value};
use tempfile::tempdir;

#[tokio::test]
async fn file_tools_reject_absolute_and_relative_paths_outside_the_workspace() {
    let parent = tempdir().unwrap();
    let workspace = parent.path().join("project");
    std::fs::create_dir(&workspace).unwrap();
    let outside = parent.path().join("outside.csv");
    std::fs::write(&outside, "name,value\noriginal,1\n").unwrap();
    let registry = ToolRegistry::with_standard_tools(&workspace);

    for path in [outside.to_str().unwrap(), "../outside.csv"] {
        for (tool, request) in [
            ("read_file", json!({"path": path})),
            ("write_file", json!({"path": path, "content": "changed"})),
            (
                "edit_file",
                json!({"path": path, "target_content": "original", "replacement_content": "changed"}),
            ),
            ("grep", json!({"path": path, "pattern": "original"})),
            ("office", json!({"action": "csv_query", "file_path": path})),
        ] {
            let error = registry
                .get(tool)
                .unwrap()
                .execute(request)
                .await
                .unwrap_err();
            assert!(
                error.to_string().contains("escapes the workspace"),
                "{tool}: {error}"
            );
        }
    }
    assert_eq!(
        std::fs::read_to_string(outside).unwrap(),
        "name,value\noriginal,1\n"
    );
    assert!(registry
        .get("write_file")
        .unwrap()
        .execute(json!({"path": "../new.txt", "content": "changed"}))
        .await
        .is_err());
    assert!(!parent.path().join("new.txt").exists());
}

#[tokio::test]
async fn unavailable_workspaces_return_tool_errors_instead_of_panicking() {
    let parent = tempdir().unwrap();
    let file_root = parent.path().join("file.txt");
    std::fs::write(&file_root, "not a directory").unwrap();

    for root in [parent.path().join("missing"), file_root] {
        let registry = ToolRegistry::with_standard_tools(root);
        for (tool, request) in [
            ("read_file", json!({"path": "sample.txt"})),
            (
                "write_file",
                json!({"path": "sample.txt", "content": "sample"}),
            ),
            (
                "edit_file",
                json!({"path": "sample.txt", "target_content": "a", "replacement_content": "b"}),
            ),
            ("grep", json!({"pattern": "sample"})),
            ("find", json!({"pattern": "sample"})),
            (
                "office",
                json!({"action": "csv_query", "file_path": "sample.csv"}),
            ),
        ] {
            assert!(
                registry.get(tool).unwrap().execute(request).await.is_err(),
                "{tool}"
            );
        }
    }
}

#[tokio::test]
async fn read_line_ranges_validate_types_and_preserve_valid_bounds() {
    let workspace = tempdir().unwrap();
    std::fs::write(workspace.path().join("sample.txt"), "one\ntwo\nthree\n").unwrap();
    std::fs::write(workspace.path().join("empty.txt"), "").unwrap();
    let tool = ToolRegistry::with_standard_tools(workspace.path())
        .get("read_file")
        .unwrap();
    for field in ["start_line", "end_line"] {
        for invalid in [json!(0), json!(-1), json!(1.5), json!("2"), Value::Null] {
            let mut request = json!({"path": "sample.txt"});
            request[field] = invalid;
            assert!(tool.execute(request).await.is_err());
        }
    }
    assert!(tool
        .execute(json!({"path": "sample.txt", "start_line": 4, "end_line": 2}))
        .await
        .is_err());
    assert_eq!(
        tool.execute(json!({"path": "sample.txt", "start_line": 2, "end_line": 99}))
            .await
            .unwrap(),
        "   2 | two\n   3 | three\n"
    );
    assert!(tool
        .execute(json!({"path": "empty.txt"}))
        .await
        .unwrap()
        .contains("only 0 lines"));
}

#[tokio::test]
async fn search_limits_reject_invalid_input_instead_of_defaulting() {
    let workspace = tempdir().unwrap();
    std::fs::write(workspace.path().join("sample.txt"), "sample\nsample\n").unwrap();
    let registry = ToolRegistry::with_standard_tools(workspace.path());
    for name in ["grep", "find"] {
        let tool = registry.get(name).unwrap();
        for invalid in [
            json!(0),
            json!(-1),
            json!(1.5),
            json!("2"),
            json!(10_001),
            Value::Null,
        ] {
            assert!(tool
                .execute(json!({"pattern": "sample", "max_results": invalid}))
                .await
                .is_err());
        }
        assert_eq!(
            tool.execute(json!({"pattern": "sample", "max_results": 1}))
                .await
                .unwrap()
                .lines()
                .count(),
            1
        );
    }
}

#[cfg(unix)]
#[tokio::test]
async fn symlinks_cannot_expose_files_outside_the_workspace() {
    let workspace = tempdir().unwrap();
    let outside = tempdir().unwrap();
    let target = outside.path().join("outside.csv");
    std::fs::write(&target, "name,value\nsecret,1\n").unwrap();
    std::os::unix::fs::symlink(&target, workspace.path().join("linked.csv")).unwrap();
    let registry = ToolRegistry::with_standard_tools(workspace.path());
    for (tool, request) in [
        ("read_file", json!({"path": "linked.csv"})),
        (
            "office",
            json!({"action": "csv_query", "file_path": "linked.csv"}),
        ),
        ("grep", json!({"pattern": "secret", "path": "linked.csv"})),
    ] {
        assert!(registry.get(tool).unwrap().execute(request).await.is_err());
    }
    let results = registry
        .get("grep")
        .unwrap()
        .execute(json!({"pattern": "secret"}))
        .await
        .unwrap();
    assert!(results.starts_with("No matches"));
}
