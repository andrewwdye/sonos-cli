gen:
	sonos-docs generate template .
	for file in src/sonos/services/gen/*; do mv "$${file}" "$${file//-/_}"; done
	sed -i '' 's/-/_/g' src/sonos/services/gen/mod.rs
