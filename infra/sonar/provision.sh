# Applies the quality gate and rule thresholds to a running SonarQube.
# The API calls target SonarQube 2025 and later, where conditions use
# gateName and rule parameters use the profile activate_rule action.
# Set SONAR_TOKEN before running. See docs/harness.md.
set -eu

host="${SONAR_HOST_URL:-http://localhost:9000}"
token="${SONAR_TOKEN:?set SONAR_TOKEN}"
project_key="${SONAR_PROJECT_KEY:-dozens-api}"
project_name="${SONAR_PROJECT_NAME:-Dozenz API}"
spec="$(dirname "$0")/quality-gate.json"
gate="$(jq -r .name "$spec")"
profile="Dozenz Rust"

api() {
  curl -fsS -u "$token:" "$@"
}

api -X POST "$host/api/projects/create" \
  --data-urlencode "project=$project_key" \
  --data-urlencode "name=$project_name" >/dev/null

api -X POST "$host/api/qualitygates/create" --data-urlencode "name=$gate" >/dev/null

# qualitygates/create copies the built-in Sonar way conditions.
# Clear them so the gate matches quality-gate.json.
api "$host/api/qualitygates/show?name=$gate" | jq -r '.conditions[].id' | while read -r id; do
  api -X POST "$host/api/qualitygates/delete_condition" --data-urlencode "id=$id" >/dev/null
done

jq -c '.conditions[]' "$spec" | while read -r condition; do
  api -X POST "$host/api/qualitygates/create_condition" \
    --data-urlencode "gateName=$gate" \
    --data-urlencode "metric=$(printf '%s' "$condition" | jq -r .metric)" \
    --data-urlencode "op=$(printf '%s' "$condition" | jq -r .op)" \
    --data-urlencode "error=$(printf '%s' "$condition" | jq -r .error)" >/dev/null
done

api -X POST "$host/api/qualitygates/select" \
  --data-urlencode "projectKey=$project_key" \
  --data-urlencode "gateName=$gate" >/dev/null

from="$(api "$host/api/qualityprofiles/search?language=rust" | jq -r '.profiles[] | select(.isDefault == true) | .key')"
profile_key="$(api -X POST "$host/api/qualityprofiles/copy" \
  --data-urlencode "fromKey=$from" \
  --data-urlencode "toName=$profile" | jq -r .key)"

api -X POST "$host/api/qualityprofiles/set_default" \
  --data-urlencode "language=rust" \
  --data-urlencode "qualityProfile=$profile" >/dev/null

jq -c '.ruleParameters[]' "$spec" | while read -r rule; do
  api -X POST "$host/api/qualityprofiles/activate_rule" \
    --data-urlencode "key=$profile_key" \
    --data-urlencode "rule=$(printf '%s' "$rule" | jq -r .key)" \
    --data-urlencode "params=$(printf '%s' "$rule" | jq -r .param)=$(printf '%s' "$rule" | jq -r .value)" >/dev/null
done

echo "applied gate '$gate' and profile '$profile' to $project_key"
