#!/usr/bin/env python3
import sys
import os
import html
import re

def highlight_content(text):
    # 1. Highlighting for AntiSlop output (Findings)
    text = re.sub(r'(CRITICAL|HIGH)', r'<span class="red">\1</span>', text)
    text = re.sub(r'(MEDIUM)', r'<span class="orange">\1</span>', text)
    text = re.sub(r'(LOW)', r'<span class="green">\1</span>', text)
    text = re.sub(r'(\[stub\]|\[deferral\]|\[hedging\]|\[noise\])', r'<span class="purple">\1</span>', text)
    text = re.sub(r'(\s+)([\w\/\.-]+:\d+:\d+:)', r'\1<span class="cyan underline">\2</span>', text)
    text = re.sub(r'(→)', r'<span class="dim">\1</span>', text)
    text = re.sub(r'(→\s+)(.*)', r'\1<span class="code">\2</span>', text)
    
    # 2. Highlighting for Help / Manpages
    # Flags: -h, --help, --version
    text = re.sub(r'(\s)(-\w|--[\w-]+)', r'\1<span class="yellow">\2</span>', text)
    # Metavars: <PATH>, <FILE>
    text = re.sub(r'(<[^>]+>)', r'<span class="cyan">\1</span>', text)
    # Headers: USAGE:, OPTIONS:
    text = re.sub(r'^([A-Z][A-Z\s]+):', r'<span class="red bold">\1:</span>', text, flags=re.MULTILINE)
    
    # 3. Highlighting for Hygiene Output
    text = re.sub(r'(✅|PASSED)', r'<span class="green bold">\1</span>', text)
    text = re.sub(r'(⚠️|WARNING)', r'<span class="orange bold">\1</span>', text)
    text = re.sub(r'(❌|FAILED)', r'<span class="red bold">\1</span>', text)
    text = re.sub(r'(Orthogonality Check)', r'<span class="purple bold">\1</span>', text)
    
    # 4. Success/Clean Output
    text = re.sub(r'(✓)', r'<span class="green bold">\1</span>', text)
    text = re.sub(r'(No AI slop detected!)', r'<span class="green">\1</span>', text)

    # 5. Lists/Bullets
    text = re.sub(r'(•)', r'<span class="cyan">\1</span>', text)
    
    # 6. Box Headers (Hygiene Survey)
    # Highlight text inside "┌─ TITLE ─...─┐"
    text = re.sub(r'(┌─\s+)([A-Z\s&]+)(\s+─+┐)', r'\1<span class="purple bold">\2</span>\3', text)
    # Highlight completion bars (█)
    text = re.sub(r'(█+)', r'<span class="green">\1</span>', text)

    # 4. Modern Block Prompt (Reliable, no broken glyphs)
    p10k_prompt_safe = (
        '<div class="prompt-line">'
        '<span class="block-blue">~/repos/antislop</span>'
        '<span class="block-green">git:main</span>'
        '<span class="prompt-char">❯</span> '
        '</div>'
    )
    
    lines = text.split('\n')
    processed_lines = []
    for line in lines:
        if line.startswith("$ "):
            cmd = line[2:]
            processed_lines.append(f'{p10k_prompt_safe}<span class="cmd">{cmd}</span>')
        else:
            processed_lines.append(line)
            
    return '\n'.join(processed_lines)

def generate_html(title, content, output_path):
    escaped_content = html.escape(content)
    highlighted_content = highlight_content(escaped_content)
    
    html_template = f"""
<!DOCTYPE html>
<html>
<head>
    <style>
        @import url('https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@500;700&display=swap');
        
        body {{
            margin: 0;
            padding: 200px;
            background-color: #121212;
            display: inline-block;
            width: fit-content;
            height: fit-content;
            font-family: 'JetBrains Mono', monospace;
        }}
        
        .terminal-window {{
            background-color: #1a1b26; /* Tokyo Night */
            border-radius: 0px;
            box-shadow: none;
            width: 880px;
            overflow: hidden;
            border: 1px solid #2f334d;
            
            /* Ensure no external interaction styles */
            outline: none !important;
        }}
        
        .terminal-header {{
            background-color: #1f2335;
            padding: 14px 20px;
            display: flex;
            align_items: center;
            border-bottom: 1px solid #2f334d;
        }}
        
        .buttons {{ display: flex; gap: 8px; }}
        .button {{ width: 12px; height: 12px; border-radius: 50%; opacity: 0.8; }}
        .close {{ background-color: #ff5f56; }}
        .minimize {{ background-color: #ffbd2e; }}
        .maximize {{ background-color: #27c93f; }}
        
        .title {{
            color: #7aa2f7;
            font-size: 13px;
            margin-left: 20px;
            flex-grow: 1;
            text-align: center;
            padding-right: 50px;
            font-weight: 700;
        }}
        
        .terminal-body {{
            padding: 24px 30px;
            color: #a9b1d6;
            font-size: 14px;
            line-height: 1.6;
            white-space: pre-wrap;
        }}
        
        /* Syntax Colors */
        .red {{ color: #f7768e; }}
        .orange {{ color: #ff9e64; }}
        .yellow {{ color: #e0af68; }}
        .green {{ color: #9ece6a; }}
        .purple {{ color: #bb9af7; }}
        .cyan {{ color: #7dcfff; }}
        .dim {{ color: #565f89; }}
        .bold {{ font-weight: bold; }}
        .underline {{ text-decoration: underline; text-underline-offset: 4px; }}
        
        .cmd {{ color: #c0caf5; font-weight: bold; margin-left: 8px; vertical-align: middle; }}
        .code {{ color: #e0af68; font-style: italic; }}
        
        /* Block Prompt Styles */
        .prompt-line {{ display: inline-flex; align-items: center; height: 26px; vertical-align: middle; margin-bottom: 2px; }}
        
        .block-blue {{ 
            background-color: #7aa2f7; color: #1a1b26; 
            padding: 2px 10px; font-weight: bold; border-radius: 4px 0 0 4px;
            margin-right: 2px;
        }}
        
        .block-green {{ 
            background-color: #9ece6a; color: #1a1b26; 
            padding: 2px 10px; font-weight: bold; border-radius: 0 4px 4px 0;
            margin-right: 8px;
        }}
        
        .prompt-char {{
            color: #f7768e; font-weight: bold; font-size: 16px;
        }}
        
    </style>
</head>
<body>
    <div class="terminal-window">
        <div class="terminal-header">
            <div class="buttons">
                <div class="button close"></div>
                <div class="button minimize"></div>
                <div class="button maximize"></div>
            </div>
            <div class="title">{title}</div>
        </div>
        <div class="terminal-body">{highlighted_content}</div>
    </div>
</body>
</html>
    """
    
    with open(output_path, 'w') as f:
        f.write(html_template)
    print(f"Generated {output_path}")

if __name__ == "__main__":
    if len(sys.argv) < 3:
        print("Usage: python3 generate_terminal_html.py <title> <output_file> [input_file]")
        sys.exit(1)
        
    title = sys.argv[1]
    output_path = sys.argv[2]
    
    if len(sys.argv) > 3:
        with open(sys.argv[3], 'r') as f:
            content = f.read()
    else:
        content = sys.stdin.read()
        
    generate_html(title, content, output_path)
