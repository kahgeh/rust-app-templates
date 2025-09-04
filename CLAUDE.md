# Datastar is a new hypermedia library 
Strictly follow the guidance below whenever building a web application, specifically one that involves web applications.

## MANDATORY WORKFLOW: Before Writing ANY Datastar Code
You MUST follow these steps IN ORDER:
1. **LIST** - First list ALL Datastar attributes you plan to use in your response
2. **VERIFY** - Use grep to search for EACH attribute in `$(git rev-parse --show-toplevel)/ref/datastar-docs/reference/attributes.md`
3. **CHECK SYNTAX** - Read the exact syntax, modifiers, and usage for each verified attribute
4. **WRITE CODE** - Only after verification, write the implementation

### Example workflow:
```
User asks: "Add a loading indicator"
You MUST:
1. Think: "I need data-indicator attribute"
2. Run: grep "data-indicator" $(git rev-parse --show-toplevel)/ref/datastar-docs/reference/attributes.md
3. Read the results to confirm it exists and understand its usage
4. Only then use it in code
```

## Common Datastar Mistakes to AVOID
- ❌ NEVER use `data-store` (DOES NOT EXIST - use `data-signals` instead)
- ❌ NEVER use `data-on-visible` (DOES NOT EXIST - check docs for visibility detection)
- ❌ NEVER guess attribute names - ALWAYS verify first
- ❌ NEVER assume an attribute exists because it "makes sense"

## Guidance
- ALWAYS understand datastar key value proposition and approaches by reading $(git rev-parse --show-toplevel)/ref/datastar-features.md and $(git rev-parse --show-toplevel)/ref/datastar-docs/guide/*
- ALWAYS verify ALL attributes exist in $(git rev-parse --show-toplevel)/ref/datastar-docs/reference/attributes.md BEFORE using them
- ALWAYS check actions in $(git rev-parse --show-toplevel)/ref/datastar-docs/reference/actions.md BEFORE using them
- NEVER trust examples without verifying - they may use outdated syntax
- When you're about to write `data-*` STOP and grep the docs first
- If grep returns no results, the attribute DOES NOT EXIST - find the correct one
- Understanding how it works under the hood will help you design something novel or debug, $(git rev-parse --show-toplevel)/ref/datastar contains the framework code

# Open props
In order to provide a sense of consistency, there should not be any hardfixes of css values. Only use custom properties(vars) from open props.
## Guidance
- ALWAYS understand open props key value proposition and approaches by reading $(git rev-parse --show-toplevel)/ref/open-props-features.html
- ALWAYS only use variables listed in $(git rev-parse --show-toplevel)/ref/open-props-features.html

# TypeDB is a new database 
## Guidance
- ALWAYS understand fully typedb v3 syntax using documents here $(git rev-parse --show-toplevel)/ref/reference/modules/ROOT/pages/typeql, before writing any typeql schema definition, data insertions or queries.
- ALWAYS understand typedb key value propositions by reading $(git rev-parse --show-toplevel)/ref/typedb-features.html when modelling with typedb

## Tools 
### Testing and accessing typedb
- In order test query, run typedb console like this : 
typedb console --address localhost:1729 --username admin --password password --tls-disabled \
--command "transaction schema <database name>" \
--command "<your query>" \
--command "commit"

also if you need to write tql file, ensure you don't have comments at the top of the file.
- Do not use hardfixes in css, always use open props variables
