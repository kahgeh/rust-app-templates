- ALWAYS understand fully typedb v3 syntax using documents here reference/modules/ROOT/pages/typeql, before writing any typeql schema definition, data insertions or queries.
- ALWAYS understand typedb key value propositions by reading typedb-features.html when modelling with typedb

# Tools 
## Testing and accessing typedb
- In order test query, run typedb console like this : 
typedb console --address localhost:1729 --username admin --password password --tls-disabled \
--command "transaction schema <database name>" \
--command "<your query>" \
--command "commit"

also if you need to write tql file, ensure you don't have comments at the top of the file.
