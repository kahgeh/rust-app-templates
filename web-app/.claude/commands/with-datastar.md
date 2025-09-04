---
description: Execute task with Datastar documentation preloaded
argument-hint: [your task or question]
---

Before working on this task, first read and understand the Datastar documentation:

1. Read the Datastar attributes reference at `$(git rev-parse --show-toplevel)/ref/datastar-docs/reference/attributes.md` to understand all available attributes and their exact syntax
2. Read the Datastar actions reference at `$(git rev-parse --show-toplevel)/ref/datastar-docs/reference/actions.md` to understand all available actions
3. Review the Datastar features guide at `$(git rev-parse --show-toplevel)/ref/datastar-features.md` to understand the framework's approach
4. Check the Datastar guide files in `$(git rev-parse --show-toplevel)/ref/datastar-docs/guide/` for best practices

After reading and understanding the documentation, proceed with the following task:

$ARGUMENTS

IMPORTANT RULES:
- NEVER guess or assume Datastar attribute names
- ALWAYS use only attributes that exist in the documentation
- If unsure about an attribute, verify it exists using: `./scripts/verify-datastar-attrs.sh [attribute-name]`
- Common mistakes to avoid:
  - Do NOT use `data-on-change` (doesn't exist - use `data-on-input` or `data-bind`)
  - Do NOT use `data-bind-checked` (doesn't exist - use `data-attr-checked` for visual state)
  - Do NOT use `data-store` (doesn't exist - use `data-signals`)
  - Do NOT use `data-on-visible` (doesn't exist - check docs for correct visibility detection)