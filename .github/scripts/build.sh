#!/bin/bash
set -e

mkdir -p public
echo "[" > public/apps.json

FIRST=true

# Find all directories that contain a Cargo.toml
for d in */ ; do
    if [ -f "$d/Cargo.toml" ]; then
        cd "$d"
        # Get package name from Cargo.toml
        NAME=$(grep '^name' Cargo.toml | head -1 | cut -d '"' -f 2)
        
        echo "Building $NAME..."
        cargo build --target wasm32-wasip1 --release
        
        # Copy the resulting .wasm file to public directory
        cp target/wasm32-wasip1/release/$NAME.wasm ../public/
        
        cd ..
        
        # Add to JSON registry
        if [ "$FIRST" = true ]; then
            FIRST=false
        else
            echo "," >> public/apps.json
        fi
        
        cat <<EOF >> public/apps.json
  {
    "id": "$NAME",
    "name": "$NAME",
    "url": "$NAME.wasm"
  }
EOF
    fi
done

echo "]" >> public/apps.json

echo "Build complete. Generated apps.json:"
cat public/apps.json
