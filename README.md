<p align="center"><strong>Kodex CLI</strong> is a coding agent from OpenAI that runs locally on your computer.
<p align="center">
  <img src="https://github.com/openai/kodex/blob/main/.github/kodex-cli-splash.png" alt="Kodex CLI splash" width="80%" />
</p>
</br>
If you want Kodex in your code editor (VS Code, Cursor, Windsurf), <a href="https://developers.openai.com/kodex/ide">install in your IDE.</a>
</br>If you want the desktop app experience, run <code>kodex app</code> or visit <a href="https://chatgpt.com/kodex?app-landing-page=true">the Kodex App page</a>.
</br>If you are looking for the <em>cloud-based agent</em> from OpenAI, <strong>Kodex Web</strong>, go to <a href="https://chatgpt.com/kodex">chatgpt.com/kodex</a>.</p>

---

## Quickstart

### Installing and running Kodex CLI

Run the following on Mac or Linux to install Kodex CLI:

```shell
curl -fsSL https://chatgpt.com/kodex/install.sh | sh
```

Run the following on Windows to install Kodex CLI:

```shell
powershell -ExecutionPolicy ByPass -c "irm https://chatgpt.com/kodex/install.ps1 | iex"
```

The standalone installers download from `https://releases.openai.com/kodex` by default and fall back to GitHub Releases if a metadata or asset download is unavailable. To force GitHub Releases, set `KODEX_INSTALLER_USE_RELEASES_OPENAI_COM` to `false` (`0` and `no` are also accepted):

```shell
curl -fsSL https://chatgpt.com/kodex/install.sh | KODEX_INSTALLER_USE_RELEASES_OPENAI_COM=false sh
```

```powershell
$env:KODEX_INSTALLER_USE_RELEASES_OPENAI_COM='false'; irm https://chatgpt.com/kodex/install.ps1 | iex
```

Kodex CLI can also be installed via the following package managers:

```shell
# Install using npm
npm install -g @openai/kodex
```

```shell
# Install using Homebrew
brew install --cask kodex
```

Then simply run `kodex` to get started.

<details>
<summary>You can also go to the <a href="https://github.com/openai/kodex/releases/latest">latest GitHub Release</a> and download the appropriate binary for your platform.</summary>

Each GitHub Release contains many executables, but in practice, you likely want one of these:

- macOS
  - Apple Silicon/arm64: `kodex-aarch64-apple-darwin.tar.gz`
  - x86_64 (older Mac hardware): `kodex-x86_64-apple-darwin.tar.gz`
- Linux
  - x86_64: `kodex-x86_64-unknown-linux-musl.tar.gz`
  - arm64: `kodex-aarch64-unknown-linux-musl.tar.gz`

Each archive contains a single entry with the platform baked into the name (e.g., `kodex-x86_64-unknown-linux-musl`), so you likely want to rename it to `kodex` after extracting it.

</details>

### Using Kodex with your ChatGPT plan

Run `kodex` and select **Sign in with ChatGPT**. We recommend signing into your ChatGPT account to use Kodex as part of your Plus, Pro, Business, Edu, or Enterprise plan. [Learn more about what's included in your ChatGPT plan](https://help.openai.com/en/articles/11369540-kodex-in-chatgpt).

You can also use Kodex with an API key, but this requires [additional setup](https://developers.openai.com/kodex/auth#sign-in-with-an-api-key).

## Docs

- [**Kodex Documentation**](https://developers.openai.com/kodex)
- [**Contributing**](./docs/contributing.md)
- [**Installing & building**](./docs/install.md)
- [**Open source fund**](./docs/open-source-fund.md)

This repository is licensed under the [Apache-2.0 License](LICENSE).
