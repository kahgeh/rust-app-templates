# Datastar Documentation Crawler

This crawler downloads and converts the Datastar documentation from https://data-star.dev into clean Markdown files for offline reference.

## Features

- **Dynamic page discovery** - Automatically discovers all pages from the navigation structure
- **HTML to Markdown conversion** - Converts documentation to clean, readable Markdown
- **Demo extraction** - Preserves Datastar demo code blocks with proper formatting
- **Character encoding fixes** - Handles UTF-8 encoding issues in the source
- **Hierarchical structure** - Maintains the documentation's organization

## Prerequisites

- Python 3.8 or higher
- pip package manager

## Setup

1. Navigate to the crawler directory:
   ```bash
   cd scripts/crawler
   ```

2. Create and activate a virtual environment:
   ```bash
   python -m venv .venv
   source .venv/bin/activate  # On Windows: .venv\Scripts\activate
   ```

3. Install dependencies:
   ```bash
   pip install -r requirements.txt
   ```

## Running the Crawler

### Basic Usage

Run the crawler to download/update all documentation:

```bash
python crawler.py
```

### Clean and Re-crawl

To remove existing docs and download fresh copies:

```bash
rm -rf ../../ref/datastar-docs/* && python crawler.py
```

### Using Virtual Environment Directly

If you're not in the activated virtual environment:

```bash
.venv/bin/python crawler.py
```

## Output

The crawler saves documentation to: `../../ref/datastar-docs/`

Directory structure:
```
ref/datastar-docs/
├── guide/           # Getting started and core concepts
├── reference/       # API reference and attributes
├── examples/        # Code examples and demos
├── how_tos/         # Tutorials and guides
└── index.json       # Index of all downloaded files
```

## How It Works

1. **Discovery Phase**: Fetches each section's index page and extracts navigation links
2. **Download Phase**: Downloads each page with rate limiting (0.3s delay)
3. **Processing Phase**: 
   - Extracts main content, removing navigation and footer
   - Converts HTML to Markdown
   - Extracts Demo sections as formatted code blocks
   - Fixes character encoding issues
4. **Index Creation**: Generates `index.json` with file metadata

## Maintenance

### Updating Documentation

Run the crawler periodically to keep documentation current:

```bash
# Weekly update (example cron job)
0 0 * * 0 cd /path/to/scripts/crawler && .venv/bin/python crawler.py
```

### Troubleshooting

If pages fail to download:
- Check your internet connection
- Verify the Datastar website is accessible
- Review failed URLs in the crawler output
- The crawler will continue even if some pages fail

### Configuration

Edit these variables in `crawler.py` to customize:

```python
BASE_URL = "https://data-star.dev"        # Source website
OUTPUT_DIR = Path("../../ref/datastar-docs")  # Output directory
MAIN_SECTIONS = ["guide", "reference", "examples", "how_tos"]  # Sections to crawl
```

## Dependencies

- `requests` - HTTP client for downloading pages
- `beautifulsoup4` - HTML parsing and navigation extraction
- `markdownify` - HTML to Markdown conversion
- `rich` - Terminal output formatting and progress bars

## Notes

- The crawler is polite to the server with built-in rate limiting
- Character encoding issues in the source are automatically fixed
- All internal and external links are preserved
- Demo sections containing `data-*` attributes are extracted as HTML code blocks