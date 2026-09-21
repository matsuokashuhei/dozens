const SKILL_URL =
  "https://raw.githubusercontent.com/cursor/plugins/main/pstack/skills/poteto-mode/SKILL.md"
const SKILLS_DIR = "https://raw.githubusercontent.com/cursor/plugins/main/pstack/skills/"

const INSTRUCTION = `<EXTREMELY_IMPORTANT>
POTETO_MODE_BOOTSTRAP

Before you act on the user's request, read this skill in full:
${SKILL_URL}

Then read every file it references before you act. Resolve a path like playbooks/feature.md under
${SKILL_URL.replace("SKILL.md", "")} and a named principle or skill like principle-model-the-domain
under ${SKILLS_DIR}. Follow poteto-mode for the rest of the session.
</EXTREMELY_IMPORTANT>`

export default {
  id: "poteto-mode",
  async setup(ctx) {
    if (typeof ctx?.session?.hook !== "function") return

    const registration = await ctx.session.hook("context", (event) => {
      const messages = event?.messages
      if (!Array.isArray(messages)) return

      const injected = messages.some((message) =>
        message?.content?.some(
          (part) => typeof part.text === "string" && part.text.includes(SKILL_URL),
        ),
      )
      const firstUser = messages.find((message) => message.role === "user")
      if (injected || !Array.isArray(firstUser?.content)) return

      firstUser.content.unshift({ type: "text", text: INSTRUCTION })
    })

    return () => registration?.dispose?.()
  },
}
