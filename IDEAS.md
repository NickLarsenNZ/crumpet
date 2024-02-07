# Ideas

- ~Support [`Jinja2`](https://docs.rs/minijinja/latest/minijinja/) for those more familiar to that~
- Use [`tera`](https://github.com/Keats/tera) as the templating engine
- Use [`gitoxide`](https://github.com/Byron/gitoxide) for git operations
- Use [`octorust`](https://docs.rs/octorust/latest/octorust/) for Github API interactions
- Support other VSC providers (eg: Gitlab, Gitea)
- Clone repos into directory structure like GHQ does (eg: `blah/github.com/org/repo/`)
- Pull request template
- Package for [`asdf`](https://asdf-vm.com/plugins/create.html)
- Package for [`cargo install`](https://doc.rust-lang.org/cargo/reference/publishing.html)
- Provide a [nix flake, use crane](https://fasterthanli.me/series/building-a-rust-service-with-nix/part-11#building-catscii-with-nix-build)
- Make a Github Action

## Questions

- [ ] Run `crumpet` at the template source (needs to know the remote repos to template, and have permission to raise PRs)
- [ ] Run `crumpet` at the destination (no tracking required, simpler permissions for raising PRs, can run locally to initialise a new repo)
- [ ] How should we "mark" templated files? Two obvious options come to mind: `file.tera.<EXT>` or `file.<EXT>.tera`
- [ ] How do we handle files which are not templated (don't include above naming convention)? Just copy them over? Should
      we make this behaviour configurable? (Yes)
- [ ] Solve all open questions about the example config file below.

## Configuration

If run a the destination repo...

- Reserve `.crumpet/` to store crumpet related files (config, PR template(s))
- `.crumpet/config.yml` example:
   ```yml
   version: blah
   template:
     source: https://github.com/example/template
     # Should we use 'ref' as the key here? Also add documentation for this key, it basically can be a tag, commit or branch.
     hash: abcdef0
   pull_request:
     enabled: true # although, what happens if you run it locally to test? Some CIs give an env var so you can tell if it is run via CI
     is_draft: false # Mark the PR as a draft
     # the following can be specified in pull request template frontmatter - which takes precedence?
     title:
      content: "chore: Apply template changes"
      # OR
      # Inline template or template file (resolves to .crumpet/templates/my-template).
      # How do we handle inline template vs template file?
      template: "chore: Apply template changes from ref:{{revision}}"
     body:
       content: My PR body text content
       # OR
       # Inline template or template file (resolves to .crumpet/templates/my-template).
       # How do we handle inline template vs template file?
       template: my-template
     labels: [size/s]
     assignees: [developers]
   ```

## Template Helper Functions

We want to provide multiple template helper functions, to easily render out common (and repetitive) text content. Having
such functions also helps users to avoid escaping curly brackets while producing other templated content (CI workflows
for example). A few of these functions come to mind:

| Input (Tera)                 | Rendered Output    | Notes                                                                                                                                  |
| ---------------------------- | ------------------ | -------------------------------------------------------------------------------------------------------------------------------------- |
| `{{ github::expr(<EXPR>) }}` | `${{ <EXPR> }}`    | -                                                                                                                                      |
| `{{ github::env(<VAR>) }}`   | `${{ env.<VAR> }}` | The same output can be achieved using `github::expr(env.<VAR>)`. This function simply provides a shorthand for such a common use-case. |
| `{{ rand::nanoid }}`         | `<NANO_ID>`        | Outputs a random Nanoid. (Do we really need this?)                                                                                     |
