# Datastar is a new hypermedia library 
Strictly follow the guidance below whenever building a web application, specifically one that involves web applications.
## Guidance
- ALWAYS understand datastar key value proposition and approaches by reading ref/datastar-features.md and ref/datastar-docs/guide/*
- ALWAYS only use attributes, actions, that are provided here ref/datastar-docs/reference/*
- Examples in ref/datastar-docs/examples/* and ref/datastar-rust/examples can be useful
- Understanding how if works under the hood will help you design something novel or debug, ref/datastar contains the framework code.

# Open props
In order to provide a sense of consistency, there should not be any hardfixes of css values. Only use custom properties(vars) from open props.
## Guidance
- ALWAYS understand open props key value proposition and approaches by reading - ref/open-props-features.html
- ALWAYS only use variables listed in - ref/open-props-features.html

# TypeDB is a new database 
## Guidance
- ALWAYS understand fully typedb v3 syntax using documents here reference/modules/ROOT/pages/typeql, before writing any typeql schema definition, data insertions or queries.
- ALWAYS understand typedb key value propositions by reading ref/typedb-features.html when modelling with typedb

## Tools 
### Testing and accessing typedb
- In order test query, run typedb console like this : 
typedb console --address localhost:1729 --username admin --password password --tls-disabled \
--command "transaction schema <database name>" \
--command "<your query>" \
--command "commit"

also if you need to write tql file, ensure you don't have comments at the top of the file.
- Do not use hardfixes in css, always use open props variables
