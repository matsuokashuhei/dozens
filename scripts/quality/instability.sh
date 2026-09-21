# Fails when a Rust file is both widely depended on and unstable.
# The stable dependencies principle wants a widely depended-on file to be
# stable. A file with many dependents and a high efferent coupling is the
# dangerous shape.
set -eu

repo="${1:-.}"
violations="$(codelore analyze --analysis instability --repo "$repo" --format json --no-banner 2>/dev/null \
  | jq -r '.[] | select(.path | endswith(".rs")) | select(.ca >= 5 and .instability > 0.5) | "\(.path) ca=\(.ca) instability=\(.instability)"')"

if [ -n "$violations" ]; then
  echo "Stable dependencies principle violations (ca >= 5 and instability > 0.5):" >&2
  echo "$violations" >&2
  exit 1
fi
echo "instability gate passed"
