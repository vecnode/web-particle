# AGENTS.md

Defines available AI agents, their responsibilities, and operational guidelines.

## Agent Guidelines

- **No New Documentation:** Do not create Markdown files explaining solutions unless explicitly requested.
- **No Emojis:** Maintain clean, professional code without emojis or strings like "...".
- **Planning Protocol:** When asked for plans (without code edits), respond in writing only before proceeding.
- **Code Respect:** Always preserve existing codebase structure and patterns during implementation.
- **Database Respect:** Always preserve existing database structure and patterns during implementation.

#### General Guidelines 

- **Scoped Changes:** Keep pull requests focused, avoiding unrelated refactors in the same commit.
- **Testing Requirement:** Run relevant checks and tests before proposing or applying code changes.
- **Clear Commits:** Write concise, descriptive commit messages that explain the purpose of each change.
- **Review Friendly:** Prefer small, incremental changes that are easy to review and reason about.
- **Security & Safety:** Avoid introducing unsafe patterns or dependencies without explicit justification.
- **Configurability:** Respect existing configuration, feature flags, and environment settings.
- **Documentation Updates:** When behavior changes, update comments and in-code documentation accordingly.
- **User Experience:** Preserve or improve existing UX patterns, layouts, and interaction flows.
- **Reversibility:** Structure changes so they can be reverted cleanly if needed.