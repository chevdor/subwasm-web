VERSION := `toml get Cargo.toml package.version | jq -r`
export TAG:=`toml get Cargo.toml "package.version" | jq -r .`

# List available commands
_default:
  just --choose --chooser "fzf +s -x --tac --cycle"

help:
	just --list

serve:
  #!/usr/bin/env bash
  trunk serve -d ./dist crates/subwasm_app/index.html

build:
  trunk build -d ./dist crates/subwasm_app/index.html

# Generate the readme as .md
md:
  #!/usr/bin/env bash
  asciidoctor -b docbook -a leveloffset=+1 -o - README_src.adoc | pandoc   --markdown-headings=atx --wrap=preserve -t markdown_strict -f docbook - > README.md
