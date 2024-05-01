# Hemlock 🌲

`hemlock` - is a CLI tool for vendoring files or directories from remote resources/repositories. It can be used for easy .proto files vendoring from multiple repositories to perform `protogen`

## ⚙️ Installation
`hemlock` can be easly installed using package manager (currently supported: brew):

```bash
brew tap MadL1me/hemlock

brew install hemlock
```

## Usage

Vendoring can be activated by calling main command:

```sh
hemlock
```

By default, hemlock is searching for `hemlock.yaml` config in same directory. 

For example, for following hemlock.yaml config:

```yaml
version: 1
vendor_dir: vendor
external_deps:
  - github.com/MadL1me/RhythmGE/src/index.scss@68f7379
```

it will create following structure:

```
vendor/
├── index.scss@68f7379
```

### Hemlock.yaml config format

```yaml
version: 1
vendor_dir: vendor
external_deps:
  - github.com/MadL1me/RhythmGE/src/index.scss@master    # would get from master branch
  - github.com/MadL1me/RhythmGE/src/index.scss@68f7379   # would get from 68f7379 hash commit
  - github.com/MadL1me/RhythmGE/src/index.scss           # would get from master branch by default

  # you can specify http/https protocol if you want
  - https://github.com/MadL1me/RhythmGE/src/index.scss@master   

  # github permalink buttons also work as well
  - http://github.com/MadL1me/RhythmGE/blob/68f7379aa960a365d7eb61577d536307334a4e2e/src/index.scss 
```

## TODO:

- [x] Basic demo with ability to vendor remote github files with concrete branch or hash commit
- [ ] Better folder structure for vendored items
- [ ] Better docs
- [ ] Ability to vendor local_files (used for protogen)
- [ ] Ability to vendor from GitLab
- [ ] Ability to vendor from BitBucket
- [ ] Ability to vendor any remote file (for example from pinterest
