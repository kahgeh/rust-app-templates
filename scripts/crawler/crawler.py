#!/usr/bin/env python3
"""
Datastar Documentation Crawler
Downloads all documentation pages from data-star.dev
"""

import json
import re
import time
from pathlib import Path
from typing import List, Dict, Set, Optional, Tuple
from urllib.parse import urljoin, urlparse

import requests
from bs4 import BeautifulSoup
from markdownify import markdownify as md
from rich.console import Console
from rich.progress import Progress, SpinnerColumn, TextColumn, BarColumn
from rich.table import Table

console = Console()

# Base configuration
BASE_URL = "https://data-star.dev"
OUTPUT_DIR = Path("../../ref/datastar-docs")

# Main documentation sections
MAIN_SECTIONS = ["guide", "reference", "examples", "how_tos"]


class DatastarCrawler:
    def __init__(self, output_dir: Path = OUTPUT_DIR):
        self.output_dir = output_dir
        self.session = requests.Session()
        self.session.headers.update({
            'User-Agent': 'Mozilla/5.0 (compatible; DocumentationCrawler/1.0)'
        })
        self.downloaded_urls: Set[str] = set()
        self.failed_urls: List[str] = []
    
    def setup_directories(self):
        """Create output directory structure."""
        for section in MAIN_SECTIONS:
            section_dir = self.output_dir / section
            section_dir.mkdir(parents=True, exist_ok=True)
    
    def build_url(self, section: str, page: str) -> str:
        """Build full URL from section and page."""
        if page:
            return f"{BASE_URL}/{section}/{page}"
        else:
            return f"{BASE_URL}/{section}"
    
    def extract_main_content(self, html: str) -> Tuple[str, str]:
        """Extract main content and title from HTML."""
        soup = BeautifulSoup(html, 'html.parser')
        
        # Extract title
        title = ""
        title_elem = soup.find('h1') or soup.find('title')
        if title_elem:
            title = title_elem.get_text(strip=True)
        
        # Remove navigation, header, footer, and other non-content elements
        for element in soup.select('header, nav, footer, aside, .sidebar, .navigation, .top, script, style, .socials'):
            element.decompose()
        
        # Find main content area
        main_content = soup.find('main') or soup.find('article') or soup.find('div', class_='content') or soup.body
        
        if not main_content:
            return title, ""
        
        # Process Demo sections to extract Datastar code
        demo_sections = main_content.find_all('fieldset', class_='demo')
        for demo in demo_sections:
            # Get the entire demo HTML
            demo_html_str = str(demo)
            
            # Check if the demo contains data-* attributes
            if 'data-' in demo_html_str:
                # Create a new element to hold the demo code
                code_container = soup.new_tag('div')
                code_container['class'] = 'demo-code-block'
                
                # Add a heading for the demo code
                demo_heading = soup.new_tag('p')
                demo_heading.string = "Demo code:"
                code_container.append(demo_heading)
                
                # Create a pre/code block with the HTML
                pre_tag = soup.new_tag('pre')
                code_tag = soup.new_tag('code')
                code_tag['data-language'] = 'html'
                code_tag['data-datastar-demo'] = 'true'
                
                # Pretty format the demo HTML
                demo_soup = BeautifulSoup(demo_html_str, 'html.parser')
                code_tag.string = demo_soup.prettify()
                
                pre_tag.append(code_tag)
                code_container.append(pre_tag)
                
                # Insert the code block after the demo section
                demo.insert_after(code_container)
        
        # Process other code blocks before conversion to preserve them
        # Look for code blocks with data-* attributes (Datastar code)
        code_blocks = main_content.find_all(['pre', 'code'])
        for i, code_block in enumerate(code_blocks):
            # Mark code blocks for preservation
            code_block['data-preserve-code'] = str(i)
            
            # Check if this contains Datastar attributes
            code_text = code_block.get_text()
            if 'data-' in code_text:
                code_block['data-datastar-code'] = 'true'
        
        return title, str(main_content)
    
    def html_to_markdown(self, html: str, title: str, url: str) -> str:
        """Convert HTML to clean Markdown with proper code block handling."""
        # Custom conversion options
        markdown = md(
            html,
            heading_style="ATX",
            bullets="-",
            code_language="html",
        )
        
        # Fix specific encoding issue: corrupted em dash
        # The source has â\x80\x93 which is a double-encoded em dash
        # This appears in the markdown as the UTF-8 bytes: c3 a2 c2 80 c2 93
        markdown = markdown.replace('â\x80\x93', '—')
        # Also fix common apostrophe corruptions
        markdown = markdown.replace('â\x80\x99', "'")
        markdown = markdown.replace('â\x80\x9c', '"')
        markdown = markdown.replace('â\x80\x9d', '"')
        
        # Add metadata header
        header = f"# {title}\n\n"
        header += f"Source: {url}\n\n"
        header += "---\n\n"
        
        # Process code blocks to ensure proper formatting
        # Look for code blocks that might contain Datastar attributes
        lines = markdown.split('\n')
        processed_lines = []
        in_code_block = False
        code_buffer = []
        
        for line in lines:
            if line.strip().startswith('```'):
                if not in_code_block:
                    in_code_block = True
                    # Check if the next content contains data- attributes
                    code_buffer = [line]
                else:
                    in_code_block = False
                    code_buffer.append(line)
                    
                    # Check if this is Datastar code
                    code_content = '\n'.join(code_buffer)
                    if 'data-' in code_content and not 'data-preserve-code' in code_content:
                        # Ensure it's marked as HTML/Datastar code
                        if code_buffer[0] == '```':
                            code_buffer[0] = '```html'
                    
                    processed_lines.extend(code_buffer)
                    code_buffer = []
            elif in_code_block:
                code_buffer.append(line)
            else:
                processed_lines.append(line)
        
        # Handle unclosed code block
        if code_buffer:
            processed_lines.extend(code_buffer)
        
        markdown = '\n'.join(processed_lines)
        
        return header + markdown
    
    def download_page(self, url: str, output_path: Path) -> bool:
        """Download a single page and convert to Markdown."""
        try:
            response = self.session.get(url, timeout=10)
            response.raise_for_status()
            
            # Extract main content
            title, content_html = self.extract_main_content(response.text)
            
            # Convert to Markdown
            markdown_content = self.html_to_markdown(content_html, title, url)
            
            # Save as Markdown file
            output_path.parent.mkdir(parents=True, exist_ok=True)
            output_path.write_text(markdown_content, encoding='utf-8')
            
            self.downloaded_urls.add(url)
            return True
        except Exception as e:
            console.print(f"[red]Failed to download {url}: {e}[/red]")
            self.failed_urls.append(url)
            return False
    
    def extract_navigation_links(self, html: str, section: str) -> List[str]:
        """Extract navigation links from the left sidebar pane."""
        soup = BeautifulSoup(html, 'html.parser')
        links = []
        
        # Multiple strategies to find navigation links
        
        # Strategy 1: Look for sidebar navigation elements
        nav_selectors = [
            'aside nav a',  # Common sidebar pattern
            'aside a',      # Fallback for aside links
            '.sidebar a',
            '.VPSidebar a',
            '.VPSidebarItem a',
            'nav.sidebar a',
            '[role="navigation"] a',
            '.sidebar-links a',
            '.sidebar-link',
        ]
        
        for selector in nav_selectors:
            for link in soup.select(selector):
                href = link.get('href', '')
                # Check if the link belongs to the current section
                if href and (href.startswith(f'/{section}/') or href == f'/{section}'):
                    # Extract page name from URL
                    if href == f'/{section}':
                        page_name = ''
                    else:
                        page_name = href.split(f'/{section}/')[-1].strip('/')
                    
                    # Filter out anchors and invalid links
                    if '#' not in page_name:
                        links.append(page_name)
        
        # Strategy 2: Find all links that match the section pattern
        if not links:
            for link in soup.find_all('a', href=True):
                href = link['href']
                if href.startswith(f'/{section}/'):
                    page_name = href.split(f'/{section}/')[-1].strip('/')
                    # Remove any query parameters or anchors
                    page_name = page_name.split('?')[0].split('#')[0]
                    if page_name:
                        links.append(page_name)
        
        # Remove duplicates and return
        unique_links = list(set(links))
        console.print(f"[cyan]Found {len(unique_links)} unique pages in {section} navigation[/cyan]")
        return unique_links
    
    def crawl_section(self, section: str, pages: List[str]) -> Dict[str, bool]:
        """Crawl all pages in a section."""
        results = {}
        
        with Progress(
            SpinnerColumn(),
            TextColumn("[progress.description]{task.description}"),
            BarColumn(),
            TextColumn("[progress.percentage]{task.percentage:>3.0f}%"),
            console=console,
        ) as progress:
            task = progress.add_task(f"[cyan]Downloading {section} pages...", total=len(pages))
            
            for page in pages:
                url = self.build_url(section, page)
                filename = f"{page}.md" if page else "index.md"
                output_path = self.output_dir / section / filename
                
                success = self.download_page(url, output_path)
                results[url] = success
                
                progress.update(task, advance=1)
                
                # Be polite to the server
                time.sleep(0.3)
        
        return results
    
    def discover_section_pages(self, section: str) -> List[str]:
        """Discover all pages in a section by fetching and parsing the section index."""
        # First, download the section index page
        index_url = f"{BASE_URL}/{section}"
        console.print(f"[cyan]Fetching navigation structure from {index_url}[/cyan]")
        
        try:
            response = self.session.get(index_url, timeout=10)
            response.raise_for_status()
            
            # Extract navigation links from the index page
            pages = self.extract_navigation_links(response.text, section)
            
            # Always ensure we have the index page itself
            if '' not in pages:
                pages.insert(0, '')
            
            return pages
            
        except Exception as e:
            console.print(f"[red]Failed to discover pages for {section}: {e}[/red]")
            console.print(f"[yellow]Falling back to index page only[/yellow]")
            return ['']  # At minimum, try to get the index page
    
    def crawl_all(self):
        """Crawl all documentation sections."""
        console.print("[bold cyan]Starting Datastar Documentation Crawler[/bold cyan]")
        console.print(f"Output directory: {self.output_dir.absolute()}")
        console.print()
        
        # Setup directories
        self.setup_directories()
        
        # Crawl each section
        for section in MAIN_SECTIONS:
            console.print(f"\n[bold]📚 Section: {section}[/bold]")
            
            # Dynamically discover pages in this section
            pages = self.discover_section_pages(section)
            
            if pages:
                # Download all discovered pages
                results = self.crawl_section(section, pages)
                
                # Report section results
                successful = sum(1 for success in results.values() if success)
                console.print(f"[green]✓ Downloaded {successful}/{len(results)} pages[/green]")
            else:
                console.print(f"[yellow]⚠ No pages found for section {section}[/yellow]")
        
        # Create index file
        self.create_index()
        
        # Final report
        self.print_summary()
    
    def create_index(self):
        """Create an index of all downloaded files."""
        index_path = self.output_dir / "index.json"
        
        index_data = {
            "sections": {},
            "total_files": 0,
            "failed_urls": self.failed_urls,
        }
        
        for section_dir in self.output_dir.iterdir():
            if section_dir.is_dir():
                files = list(section_dir.glob("*.md"))
                index_data["sections"][section_dir.name] = [f.name for f in files]
                index_data["total_files"] += len(files)
        
        index_path.write_text(json.dumps(index_data, indent=2))
        console.print(f"\n[green]Index created at: {index_path}[/green]")
    
    def print_summary(self):
        """Print a summary table of the crawl results."""
        table = Table(title="Crawl Summary", show_header=True)
        table.add_column("Section", style="cyan")
        table.add_column("Files", justify="right", style="green")
        
        total = 0
        for section_dir in sorted(self.output_dir.iterdir()):
            if section_dir.is_dir():
                file_count = len(list(section_dir.glob("*.md")))
                table.add_row(section_dir.name, str(file_count))
                total += file_count
        
        table.add_row("[bold]Total[/bold]", f"[bold]{total}[/bold]")
        
        console.print()
        console.print(table)
        
        if self.failed_urls:
            console.print(f"\n[red]Failed URLs ({len(self.failed_urls)}):[/red]")
            for url in self.failed_urls:
                console.print(f"  - {url}")


def main():
    """Main entry point."""
    crawler = DatastarCrawler()
    
    try:
        crawler.crawl_all()
    except KeyboardInterrupt:
        console.print("\n[yellow]Crawl interrupted by user[/yellow]")
    except Exception as e:
        console.print(f"\n[red]Error: {e}[/red]")
        raise


if __name__ == "__main__":
    main()