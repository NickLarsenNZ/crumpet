# Ideas

- ~~Support [`Jinja2`](https://docs.rs/minijinja/latest/minijinja/) for those more familiar to that~~
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
  - @Techassi: Good point. For files without extensions, this doesn't matter. We only need to adjust our stripping logic based on the presence of an extension
- [ ] How do we handle files which are not templated (don't include above naming convention)? Just copy them over? Should
      we make this behaviour configurable? (Yes)
  - @NickLarsenNZ: I think just copied, because they might be static files that are still content templates
- [ ] Solve all open questions about the example config file below.

## Commands

- Have a `crumpet check` command, which validates that all templated files are up-to-date and were additionally not
  modified by something outside of crumpet (for example manually changed by a developer). This could go hand-in-hand
  with a local `.crumpetignore` file, which can exclude templated files from being checked / templated.

## Configuration

If run a the destination repo...

- Reserve `.crumpet/` to store crumpet related files (config, PR template(s))
- `.crumpet/config.yml` example:
   ```yaml
    # What casing should we use for the keys? Kubernetes uses camelCase
    # and Rust uses snake_case. Serde supports both and more.
    # @NickLarsenNZ: I vote for snake_case (as per below), but if you feel
    # strongly about camelCase, I'm not opposed.

    # Version of what? Crumpet? If so, how do we handle version conflicts?
    # @NickLarsenNZ: version of the crumpet config schema (in case we want
    # to make major changes to it).
    version: 1.0-alpha

    template:
      # We might want to support different backends here. The most
      # obvious one is a remote Git repo referenced by a URL. We
      # also need to support the git+ssh scheme. Additionally, we
      # should allow referencing a (local) file path here.
      source: https://github.com/example/template

      # @NickLarsenNZ: I think it is better to have a separate key 
      # for the path, rather than adding to the url above, but if we
      # support local paths for the source, then this feels a little
      # redundant. Unless the source should be the root because it
      # contains some metadata aside from the template files.
      template_directory: my_template # defaults to template

      # Add documentation for this key, it basically can be a tag,
      # commit or branch - a commitish. When we also support local
      # file paths, do we just ignore this key? Or de we use some
      # kind of complex enum.
      # @NickLarsenNZ: Good question. Maybe we should make the template
      # `source` key contain everything necessary for whatever backends.
      # eg: 
      #   source: https://github.com/example/template.git//my_template#abcdef
      #   source: git@github.com:example/template.git//my_template#abcdef
      #   source: ./my_template
      # Here are some examples from Terraform:
      # - https://developer.hashicorp.com/terraform/language/modules/sources#github
      # - https://developer.hashicorp.com/terraform/language/modules/sources#generic-git-repository
      # - https://developer.hashicorp.com/terraform/language/modules/sources#modules-in-package-sub-directories
      # Or, we could change the `source` key to `git`, `directory`, etc.. with `ref` being exclusive to the relevant backends.
      ref: abcdef0

    pull_request:
      # Although, what happens if you run it locally to test? Some
      # CIs give an env var so you can tell if it is run via CI.
      # Github: CI=true: https://docs.github.com/en/actions/learn-github-actions/variables#default-environment-variables
      # Gitlab: CI=true: https://docs.gitlab.com/ee/ci/variables/predefined_variables.html
      enabled: true

      # Mark the pull request as a draft (defult to false)
      is_draft: false

      # The following can be specified in pull request template
      # frontmatter - which takes precedence?
      # 
      # @Techassi: I would argue that the setting in this config
      # file takes precedence.
      # @NickLarsenNZ: I would have thought the frontmatter overrides
      # a general config, but I don't mind either way if it is documented
      # 
      # One can provide a string here to be used as the inline
      # template, or the explicit keys below:
      title:
        template: "chore: Apply template changes from ref:{{revision}}"

        # OR template file (resolves to .crumpet/templates/title-template).
        template_file: title-template

      # One can provide a string here to be used as the inline
      # template, or the explicit keys below:
      body:
        template: |
          > [!IMPORTANT]
          > This Pull Request was automatically generated.
          > Please double check the changes are safe before merging

        # OR template file (resolves to .crumpet/templates/body-template).
        template_file: body-template

      # Some Git platforms support labels/tags to be attached to the created
      # pull/merge request.
      #
      # @Techassi: Suggestions: We should add 'tags' as an alias for this key.
      # @NickLarsenNZ: was about to write the same comment.
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

## High-Level Module Overview

- `git`: This module contains code to handle fetching template files from remote Git repositories. Internally it uses
  the `gitoxide` crate.
- `template`: This module contains code to render templated files. The code will render files in parallel (async). For
  templating it uses the `tera` crate. It also provides the helper functions and the render context. This module doesn't
  handle filesystem operations like globbing, reading, and writing data.
- `fs`: This module handles globbing and async operations like reading and writing files.
- `providers`:
  - `github`
  - ...
