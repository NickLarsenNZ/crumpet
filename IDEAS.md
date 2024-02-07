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
- Package for [`cargo binstall`](https://github.com/cargo-bins/cargo-binstall)
- Provide a [nix flake, use crane](https://fasterthanli.me/series/building-a-rust-service-with-nix/part-11#building-catscii-with-nix-build)
- Make a Github Action

## Questions

- [ ] Run `crumpet` at the template source (needs to know the remote repos to template, and have permission to raise PRs)
- [ ] Run `crumpet` at the destination (no tracking required, simpler permissions for raising PRs, can run locally to initialise a new repo)
- [ ] How should we "mark" templated files? Two obvious options come to mind: `file.tera.<EXT>` or `file.<EXT>.tera`
  - @NickLarsenNZ: I think both should be valid. The former allows for better syntax highlighting.
- [ ] How do we handle files which are not templated (don't include above naming convention)? Just copy them over? Should
      we make this behaviour configurable? (Yes)
  - @NickLarsenNZ: I think just copied, because they might be static files that are still content templates
- [ ] Solve all open questions about the example config file below.

## Configuration

If run a the destination repo...

- Reserve `.crumpet/` to store crumpet related files (config, PR template(s))
- `.crumpet/config.yml` example:
   ```yml
   version: blah
   template:
     source: https://github.com/example/template
     # Add documentation for this key, it basically can be a tag, commit or branch. A commitish
     ref: abcdef0
   pull_request:
     enabled: true # although, what happens if you run it locally to test? Some CIs give an env var so you can tell if it is run via CI
     is_draft: false # Mark the PR as a draft
     # the following can be specified in pull request template frontmatter - which takes precedence?
     title: # can provide a string here to be used as the inline tempalte, or the explicit keys below:
       template: "chore: Apply template changes from ref:{{revision}}"
       # OR template file (resolves to .crumpet/templates/title-template).
       template_file: my-template
     body: # can provide a string here to be used as the inline tempalte, or the explicit keys below:
       template: Please double check the changes are safe before merging
       # OR template file (resolves to .crumpet/templates/body-template).
       template_file: body-template
     labels: [size/s]
     assignees: [developers]
   ```

## Template Helper Functions

We want to provide multiple template helper functions, to easily render out common (and repetitive) text content. Having
such functions also helps users to avoid escaping curly brackets while producing other templated content (CI workflows
for example). A few of these functions come to mind:

| Input (Tera)                 | Rendered Output    | Notes                                                                                                                                  |
| :--------------------------- | :----------------- | -------------------------------------------------------------------------------------------------------------------------------------- |
| `{{ github::expr(<EXPR>) }}` | `${{ <EXPR> }}`    | -                                                                                                                                      |
| `{{ github::env(<VAR>) }}`   | `${{ env.<VAR> }}` | The same output can be achieved using `github::expr(env.<VAR>)`. This function simply provides a shorthand for such a common use-case. |
| `{{ rand::nanoid }}`         | `<NANO_ID>`        | Outputs a random Nanoid. (Do we really need this?)                                                                                     |
