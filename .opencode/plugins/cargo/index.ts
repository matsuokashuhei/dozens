import { rewriteCargoCommand } from "./rewrite.ts"

// Local V2 plugin. OpenCode resolves no node_modules for local plugins, so importing `@opencode/plugin` fails.
export default {
  id: "cargo",
  async setup(ctx) {
    const repoRoot = ctx.location?.project?.canonical ?? ctx.location?.directory
    if (!repoRoot) throw new Error("cargo: no repo root in plugin location")

    const registration = await ctx.shell.hook("create.before", (event) => {
      event.command = rewriteCargoCommand(event.command, repoRoot)
    })

    return () => registration.dispose()
  },
}
