// Match cargo only as a command word, so a path such as ~/.cargo/bin/cargo is left alone.
const CARGO_WORD = /(^|[\s;&|(])cargo(?=\s|$)/g

const RUNNER = "run --rm api cargo"

export function rewriteCargoCommand(command: string, repoRoot: string): string {
  if (command.includes(RUNNER)) return command

  const runner = `docker compose -f "${repoRoot}/apps/compose.yml" -f "${repoRoot}/apps/.devcontainer/compose.yml" ${RUNNER}`

  return command.replace(CARGO_WORD, (_match, lead: string) => `${lead}${runner}`)
}
