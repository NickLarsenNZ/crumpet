# Ideas

- Support [`Jinja2`](https://docs.rs/minijinja/latest/minijinja/) for those more familiar to that
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

## Question

- [ ] Run `crumpet` at the template source (needs to know the remote repos to template, and have permission to raise PRs)
- [ ] Run `crumpet` at the destination (no tracking required, simpler permissions for raising PRs, can run locally to initialise a new repo)

## Configuration

If run a the destination repo...

- Reserve `.crumpet/` to store crumpet related files (config, PR template(s))
- `.crumpet/config.yml` example:
   ```yml
   version: blah
   template:
     source: https://github.com/example/template
     hash: abcdef0 # or main
   pull_request:
     enabled: true # although, what happens if you run it locally to test? Some CIs give an env var so you can tell if it is run via CI
     template: pr.md # relative to .crumpet/
     # the following can be specified in pull request template frontmatter - which takes precedence?
     title: chore: Apply template ref:{{revision}}
     labels: [size/s]
     assignees: [developers]
   ```
