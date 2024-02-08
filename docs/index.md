# crumpet

## Configuration

### Example Configuration File

```yaml linenums="1" title=".crumpet/config.yml"
# What casing should we use for the keys? Kubernetes uses camelCase
# and Rust uses snake_case. Serde supports both and more.

# Version of what? Crumpet? If so, how do we handle version conflicts?
version: blah

template:
  # We might want to support different backends here. The most
  # obvious one is a remote Git repo referenced by a URL. We
  # also need to support the git+ssh scheme. Additionally, we
  # should allow referencing a (local) file path here.
  source: https://github.com/example/template

  # Add documentation for this key, it basically can be a tag,
  # commit or branch - a commitish. When we also support local
  # file paths, do we just ignore this key? Or de we use some
  # kind of complex enum.
  ref: abcdef0

pull_request:
  # Although, what happens if you run it locally to test? Some
  # CIs give an env var so you can tell if it is run via CI.
  enabled: true

  # Mark the pull request as a draft
  is_draft: false

  # The following can be specified in pull request template
  # frontmatter - which takes precedence?
  # 
  # @Techassi: I would argue that the setting in this config
  # file takes precedence.
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
    template: Please double check the changes are safe before merging

    # OR template file (resolves to .crumpet/templates/body-template).
    template_file: body-template

  # Some Git platforms support labels/tags to be attached to the created
  # pull/merge request.
  #
  # @Techassi: Suggestions: We should add 'tags' as an alias for this key.
  labels: [size/s]

  assignees: [developers]
```
