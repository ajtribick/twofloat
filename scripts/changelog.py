import argparse
import re
import sys

def get_crate_version():
    version_regex = re.compile(r'version\s*=\s*"([0-9]+\.[0-9]+\.[0-9]+)\"')
    with open('Cargo.toml', 'rt', encoding='utf-8') as f:
        for line in f:
            m = version_regex.match(line)
            if m:
                print(f"Cargo version is {m.group(1)}")
                return m.group(1)
    raise RuntimeError("Could not determine version from Cargo.toml")

def get_changelog_section(out, crate_version):
    section_regex = re.compile(r'## Version ([0-9]+\.[0-9]+(?:\.[0-9]+)?)')
    parse_state = 'begin'
    with open('CHANGELOG.md', 'rt', encoding='utf-8') as f:
        for line in f:
            line = line.rstrip()
            if parse_state == 'begin':
                m = section_regex.match(line)
                if m:
                    if m.group(0) != line:
                        raise RuntimeError("Most recent version is not final")
                    version = m.group(1)
                    splits = len([c for c in version if c == '.'])
                    version += '.0' * (2 - splits)
                    if version != crate_version:
                        raise RuntimeError("Mismatched crate and changelog versions")
                    parse_state = 'output'
            elif parse_state == 'output':
                if line.startswith('##'):
                    return
                if line:
                    print(line, file=out)
    if parse_state == 'begin':
        raise RuntimeError("Could not find relevant changelog section")

parser = argparse.ArgumentParser()
parser.add_argument('-o', '--output-file', required=True)

args = parser.parse_args()

try:
    crate_version = get_crate_version()

    with open(args.output_file, 'wt', encoding='utf-8') as f:
        get_changelog_section(f, crate_version)

except Exception as ex:
    print("Failed: {ex}", file=sys.stderr)
    sys.exit(1)
