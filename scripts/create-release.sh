#!/bin/bash

# OrangeNote CLI - Release Creation Script
# This script helps create a new release with proper versioning and tagging

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}   OrangeNote CLI - Release Creator    ${NC}"
echo -e "${BLUE}========================================${NC}"
echo

# Check if we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    echo -e "${RED}Error: Not in a git repository${NC}"
    exit 1
fi

# Get current version from Cargo.toml
CURRENT_VERSION=$(grep '^version' "$PROJECT_ROOT/Cargo.toml" | head -1 | sed 's/.*"\(.*\)".*/\1/')
echo -e "${GREEN}Current version:${NC} $CURRENT_VERSION"

# Ask for new version or use current
read -p "Enter new version (or press Enter to use $CURRENT_VERSION): " NEW_VERSION
NEW_VERSION=${NEW_VERSION:-$CURRENT_VERSION}

echo
echo -e "${YELLOW}Creating release v$NEW_VERSION${NC}"
echo

# Confirm
read -p "Proceed? (y/N): " CONFIRM
if [[ ! "$CONFIRM" =~ ^[Yy]$ ]]; then
    echo "Aborted."
    exit 0
fi

# Update version in Cargo.toml if different
if [ "$NEW_VERSION" != "$CURRENT_VERSION" ]; then
    echo -e "${BLUE}Updating Cargo.toml...${NC}"
    sed -i.bak "s/^version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/" "$PROJECT_ROOT/Cargo.toml"
    rm -f "$PROJECT_ROOT/Cargo.toml.bak"

    # Update Cargo.lock
    cd "$PROJECT_ROOT"
    cargo update -p orangenote-cli
fi

# Check for uncommitted changes
if ! git diff --quiet; then
    echo -e "${YELLOW}Uncommitted changes detected. Committing...${NC}"
    git add -A
    git commit -m "Release v$NEW_VERSION"
fi

# Create and push tag
echo -e "${BLUE}Creating tag v$NEW_VERSION...${NC}"
git tag -a "v$NEW_VERSION" -m "Release v$NEW_VERSION

See CHANGELOG.md for details."

echo
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}   Release v$NEW_VERSION prepared!     ${NC}"
echo -e "${GREEN}========================================${NC}"
echo
echo -e "To complete the release, run:"
echo -e "${YELLOW}  git push origin main${NC}"
echo -e "${YELLOW}  git push origin v$NEW_VERSION${NC}"
echo
echo -e "This will trigger GitHub Actions to:"
echo -e "  1. Build binaries for all platforms"
echo -e "  2. Create a GitHub Release"
echo -e "  3. Update the Homebrew formula"
echo
echo -e "${BLUE}After the release is published:${NC}"
echo -e "  Users can install with:"
echo -e "  ${GREEN}brew tap rodnoy/orangenote${NC}"
echo -e "  ${GREEN}brew install orangenote-cli${NC}"
