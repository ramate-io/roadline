# Site Deployment

This directory contains documentation for deploying the Roadline site.

## Overview

The Roadline site is deployed using Netlify with a pre-commit hook system that builds the site when needed.

## Deployment Process

### 1. Automatic Deployment

1. **Make changes** to the site code in `site/leptos-bevy/`
2. **Commit with build trigger**:
   ```bash
   git commit -m "ci(pre-commit: build-site-if): your changes"
   ```
3. **Pre-commit hook builds** the site and creates `dist.zip`
4. **Netlify automatically deploys** the built files

### 2. Manual Deployment

If you need to deploy manually:

1. **Build the site**:
   ```bash
   cd site/leptos-bevy
   trunk build --release
   cp _redirects dist/
   ```

2. **Create deployment package**:
   ```bash
   cd dist
   zip -r ../dist.zip . -q
   cd ..
   ```

3. **Commit the package**:
   ```bash
   git add dist.zip
   git commit -m "ci(pre-commit: build-site-if): manual deployment"
   ```

## Configuration

### `netlify.toml`

Located in the repository root:

```toml
[build]
  command = "unzip -o site/leptos-bevy/dist.zip -d site/leptos-bevy/dist"
  publish = "site/leptos-bevy/dist"

[[redirects]]
  from = "/*"
  to = "/index.html"
  status = 200
```

### `_redirects`

Located in `site/leptos-bevy/` for client-side routing:

```
/*    /index.html   200
```

## Pre-commit Hook

The deployment relies on a pre-commit hook that:

- **Triggers on**: `ci(pre-commit: build-site-if): <message>` in commit message
- **Builds site**: Using `trunk build --release`
- **Copies redirects**: From `_redirects` to `dist/`
- **Creates dist.zip**: Contains all built files
- **Adds to `git`**: Stages `dist.zip` for commit

## Troubleshooting

### Build Failures

1. **Check Rust toolchain**: `rustup show`
2. **Check WASM target**: `rustup target list --installed | grep wasm32`
3. **Check Trunk**: `which trunk`

### Deployment Issues

1. **Check Netlify logs** in the Netlify dashboard
2. **Verify `dist.zip`** is committed: `git ls-files | grep dist.zip`
3. **Check file paths** in `netlify.toml`

### Hook Issues

1. **Check hook installation**: `ls -la .git/hooks/commit-msg`
2. **Check script permissions**: `ls -la .githooks/lib-commit-msg/`
3. **Run hook manually** to debug
