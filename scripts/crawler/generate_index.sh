#!/bin/bash

# Script to generate an index.md file with a listing of all markdown files in a directory
# Usage: ./generate_index.sh <directory>

if [ $# -eq 0 ]; then
    echo "Usage: $0 <directory>"
    echo "Example: $0 ../ref/datastar-docs/examples"
    exit 1
fi

DIR="$1"
INDEX_FILE="$DIR/index.md"

if [ ! -d "$DIR" ]; then
    echo "Error: Directory '$DIR' does not exist"
    exit 1
fi

# Create temporary file for new content
TEMP_FILE=$(mktemp)

# Write header
cat > "$TEMP_FILE" << 'EOF'
# Examples Index

This is an automatically generated index of all examples in this directory.

EOF

# Get all markdown files except index.md, sort them alphabetically
for file in $(find "$DIR" -maxdepth 1 -name "*.md" -type f ! -name "index.md" | sort); do
    filename=$(basename "$file")
    # Remove .md extension for display name
    name="${filename%.md}"
    # Convert underscores to spaces and capitalize words for better display
    display_name=$(echo "$name" | sed 's/_/ /g' | awk '{for(i=1;i<=NF;i++)sub(/./,toupper(substr($i,1,1)),$i)}1')
    
    # Extract first non-empty, non-header line as description (if exists)
    description=""
    while IFS= read -r line; do
        # Skip empty lines and headers
        if [[ ! -z "$line" ]] && [[ ! "$line" =~ ^#+ ]] && [[ ! "$line" =~ ^--- ]]; then
            # Trim the line and use as description (max 100 chars)
            description=$(echo "$line" | sed 's/^[[:space:]]*//;s/[[:space:]]*$//')
            if [ ${#description} -gt 100 ]; then
                description="${description:0:97}..."
            fi
            break
        fi
    done < "$file"
    
    # Write the entry to index
    echo "## [$display_name]($filename)" >> "$TEMP_FILE"
    if [ ! -z "$description" ]; then
        echo "$description" >> "$TEMP_FILE"
    fi
    echo "" >> "$TEMP_FILE"
done

# Add footer with generation timestamp
cat >> "$TEMP_FILE" << EOF

---

*Generated on $(date '+%Y-%m-%d %H:%M:%S')*
EOF

# Move temp file to actual index file
mv "$TEMP_FILE" "$INDEX_FILE"

echo "Index file generated at: $INDEX_FILE"
echo "Total files indexed: $(find "$DIR" -maxdepth 1 -name "*.md" -type f ! -name "index.md" | wc -l | tr -d ' ')"