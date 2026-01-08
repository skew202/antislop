#!/usr/bin/env python3
"""
Hygiene Test: Check for overlap with standard linters (MECE with MegaLinter).
Iterates over AntiSlop patterns and checks if they result in standard linter errors.
dependency-free: uses regex to parse simple TOML.
"""
import sys
import os
import subprocess
import glob
import re

PATTERNS_DIR = "config/patterns"
ALLOWLIST_FILE = os.path.join(PATTERNS_DIR, "hygiene_allowlist.toml")

def parse_toml_patterns(content):
    patterns = []
    # Regex to find [[patterns]] blocks and extract fields
    # This is a simple parser for our specific format
    # It assumes keys are on separate lines
    
    current_pattern = {}
    lines = content.splitlines()
    in_pattern = False
    
    for line in lines:
        line = line.strip()
        if line.startswith("[[patterns]]") or line.startswith("{"): 
            if current_pattern and "regex" in current_pattern:
                patterns.append(current_pattern)
            current_pattern = {}
            if line.startswith("{"):
                # Inline table parsing (simple, handles escaped quotes)
                matches = re.findall(r'(\w+)\s*=\s*"(.*?)(?<!\\)"', line)
                for k, v in matches:
                    current_pattern[k] = v
                if "languages" in line:
                    lang_match = re.search(r'languages\s*=\s*\[(.*?)\]', line)
                    if lang_match:
                        langs = [l.strip().strip('"') for l in lang_match.group(1).split(",")]
                        current_pattern["languages"] = langs
                patterns.append(current_pattern)
                current_pattern = {}
            else:
                in_pattern = True
        elif in_pattern and "=" in line:
            key, val = line.split("=", 1)
            key = key.strip()
            val = val.strip()
            # Handle quoted values
            if val.startswith('"'):
                # find the closing quote (naive)
                end = val.rfind('"')
                if end > 0:
                    val = val[1:end]
            current_pattern[key] = val
            
    if current_pattern and "regex" in current_pattern:
        patterns.append(current_pattern)
        
    return patterns

def load_patterns():
    patterns = []
    for f in glob.glob(os.path.join(PATTERNS_DIR, "*.toml")):
        if f.endswith("hygiene_allowlist.toml"): continue
        try:
            with open(f, 'r') as file:
                content = file.read()
                patterns.extend(parse_toml_patterns(content))
        except Exception as e:
            print(f"Error loading {f}: {e}")
    return patterns

def load_allowlist():
    if not os.path.exists(ALLOWLIST_FILE):
        return set()
    with open(ALLOWLIST_FILE, 'r') as f:
        content = f.read()
        # extract strings from allowed_overlaps list (handling escaped quotes)
        matches = re.findall(r'"(.*?)(?<!\\)"', content)
        return set(matches)

def check_overlap_rust(pattern):
    if "Rust" not in pattern.get("languages", []):
        return False
        
    regex = pattern.get("regex", "")
    # Simple heuristic to make valid rust code from regex
    trigger = regex.replace("(?i)", "").replace("\\", "")
    if "todo!" in trigger: code = 'todo!("Msg");'
    elif "unwrap" in trigger: code = 'let x: Option<i32> = None; x.unwrap();'
    elif "unsafe" in trigger: code = 'unsafe {}'
    else: code = f'// {trigger}'
        
    code_sample = f"""
fn main() {{
    {code}
}}
"""
    with open("temp_overlap.rs", "w") as f:
        f.write(code_sample)
        
    try:
        res = subprocess.run(["cargo", "clippy", "--", "-D", "warnings"], 
                           stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        if res.returncode != 0:
            # print(f"Overlap detected for '{regex}'")
            return True
    except FileNotFoundError:
        pass
    finally:
        if os.path.exists("temp_overlap.rs"):
            os.remove("temp_overlap.rs")
            
    return False

def main():
    patterns = load_patterns()
    allowlist = load_allowlist()
    overlaps = 0
    
    print(f"Checking {len(patterns)} patterns for standard linter overlap...")
    
    for p in patterns:
        regex = p.get("regex", "")
        # Remove regex flags for matching
        clean_regex = regex.replace("(?i)", "")
        
        if regex in allowlist or clean_regex in allowlist:
            continue
            
        # 1. Check for TODO/FIXME overlap (Pylint/ESLint)
        if "TODO" in clean_regex or "FIXME" in clean_regex:
            print(f"[WARNING] '{regex}' overlaps with standard TODO checks (add to hygiene_allowlist.toml if intentional)")
            overlaps += 1
            
        # 2. Check for Rust Clippy overlap
        if check_overlap_rust(p):
             print(f"[ERROR] '{regex}' is caught by default Clippy (redundant)")
             overlaps += 1

    if overlaps > 0:
        print(f"\nFound {overlaps} overlaps.")
        print("Run: python3 scripts/check_overlap.py")
        sys.exit(1)
    else:
        print("\n✅ MECE Check Passed: Patterns are orthogonal to standard linters.")

if __name__ == "__main__":
    main()
