#!/bin/bash

set -e

#VERSION="v0.4.1"
#HOMEBREW_ACCESS_TOKEN="${ACCESS_TOKEN}"

REPOSITORY="quarylabs/sqruff"
HOMEBREW_URL="quarylabs/homebrew-quary"
API_URL="https://api.github.com/repos/$REPOSITORY/releases/tags/$VERSION"

RELEASE_VERSION=$VERSION
STRIPPED_VERSION=${RELEASE_VERSION#v}

git clone \
  --depth=1 \
  --branch=main \
  https://quarylabs:"${HOMEBREW_ACCESS_TOKEN}"@github.com/$HOMEBREW_URL \

cd homebrew-quary
git config --global user.name "GitHub Actions"
git config --global user.email "actions@github.com"

git checkout -b update-sqruff-to-"${STRIPPED_VERSION}"

sed -i "s/version \".*\"/version \"${STRIPPED_VERSION}\"/" sqruff.rb

# Get the urls and shas for the assets in the release

response=$(curl -s -H "Authorization: Bearer $HOMEBREW_ACCESS_TOKEN" -H "Accept: application/vnd.github.v3+json" "$API_URL")
echo $response

assets=$(echo "$response" | jq -c '.assets[] | {name: .name, url: .url, sha256: .browser_download_url}')
echo $assets

while read -r asset; do
  echo "Asset: $asset"
  name=$(echo "$asset" | jq -r '.name')
  url=$(echo "$asset" | jq -r '.url')
  sha256=$(echo "$asset" | jq -r '.sha256')

  if [[ "$name" == *"darwin-aarch64"* ]]; then
    DARWIN_AARCH64_SHA256_URL=$sha256
    DARWIN_AARCH64_URL="${DARWIN_AARCH64_SHA256_URL%.sha256}"
    DARWIN_AARCH64_SHA256=$(curl -sL "$sha256")
    DARWIN_AARCH64_SHA256="${DARWIN_AARCH64_SHA256%% *}"
  elif [[ "$name" == *"darwin-x86_64"* ]]; then
    DARWIN_X86_64_SHA256_URL=$sha256
    DARWIN_X86_64_URL="${DARWIN_X86_64_SHA256_URL%.sha256}"
    DARWIN_X86_64_SHA256=$(curl -sL "$sha256")
    DARWIN_X86_64_SHA256="${DARWIN_X86_64_SHA256%% *}"
  elif [[ "$name" == *"linux-x86_64-musl"* ]]; then
    LINUX_X86_64_SHA256_URL=$sha256
    LINUX_X86_64_URL="${LINUX_X86_64_SHA256_URL%.sha256}"
    LINUX_X86_64_SHA256=$(curl -sL "$sha256")
    LINUX_X86_64_SHA256="${LINUX_X86_64_SHA256%% *}"
  fi
done <<< "$assets"

# Print the assigned variables
echo "DARWIN_AARCH64_URL: $DARWIN_AARCH64_URL"
echo "DARWIN_AARCH64_SHA256: $DARWIN_AARCH64_SHA256"
echo "DARWIN_X86_64_URL: $DARWIN_X86_64_URL"
echo "DARWIN_X86_64_SHA256: $DARWIN_X86_64_SHA256"
echo "LINUX_X86_64_URL: $LINUX_X86_64_URL"
echo "LINUX_X86_64_SHA256: $LINUX_X86_64_SHA256"


sed -i "s|url \".*\"|url \"${DARWIN_AARCH64_URL}\"|" sqruff.rb
sed -i "s/sha256 \".*\"/sha256 \"${DARWIN_AARCH64_SHA256}\"/" sqruff.rb

sed -i "s|url \".*\"|url \"${DARWIN_X86_64_URL}\"|" sqruff.rb
sed -i "s/sha256 \".*\"/sha256 \"${DARWIN_X86_64_SHA256}\"/" sqruff.rb

sed -i "s|url \".*\"|url \"${LINUX_X86_64_URL}\"|" sqruff.rb
sed -i "s/sha256 \".*\"/sha256 \"${LINUX_X86_64_SHA256}\"/" sqruff.rb

git add sqruff.rb
git commit -m "Update Sqruff to version ${STRIPPED_VERSION}"
git push origin update-sqruff-to-"${STRIPPED_VERSION}"

# TODO - Create a PR using the GitHub CLI
# Store the PAT in a file that can be accessed by the
# GitHub CLI.
#echo "${{ secrets.HOMEBREW_ACCESS_TOKEN }}" > token.txt
#gh auth login --with-token < token.txt
#gh pr create --title "Update Sqruff to version ${STRIPPED_VERSION}" --body "Automated PR to update Sqruff to version ${STRIPPED_VERSION}" --base main
