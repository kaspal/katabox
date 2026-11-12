    // ---- Node types (ADR 0025): JSON <-> ListNode / TreeNode ----------------------------
    // A list travels as its values in order; a tree as its level order with null for a
    // missing child (children of a missing node are not listed, trailing nulls trimmed).
    // The solution's starter code defines `ListNode` and `TreeNode` at crate root. Building
    // and writing are iterative, so deep structures are fine; a `Box` list cannot cycle,
    // an `Rc` tree can, and a node reached twice is refused.

    const NODE_LIMIT: usize = 1_000_000;

    /// Errors from writing nodes carry their message code before ": ".
    fn node_error(e: String) -> (&'static str, String) {
        if let Some(t) = e.strip_prefix("node_cycle: ") {
            ("node_cycle", t.to_string())
        } else if let Some(t) = e.strip_prefix("node_limit: ") {
            ("return_not_serializable", t.to_string())
        } else {
            (
                "return_not_serializable",
                format!("return value cannot be converted to JSON: {e}"),
            )
        }
    }
