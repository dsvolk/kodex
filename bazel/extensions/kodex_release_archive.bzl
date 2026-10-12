"""Bazel module extension for pinned Kodex release archives."""

_KODEX_RELEASE_BUILD_FILE = """\
package(default_visibility = ["//visibility:public"])

filegroup(
    name = "kodex",
    srcs = [{entrypoint}],
)

filegroup(
    name = "package",
    srcs = glob([
        "kodex-package.json",
        {binaries},
        {resources},
        {path},
    ]),
)
"""

def _kodex_release_repository_impl(repository_ctx):
    asset = "kodex-package-x86_64-unknown-linux-musl.tar.gz"
    version = repository_ctx.attr.version
    repository_ctx.download_and_extract(
        url = [
            "https://releases.openai.com/kodex/releases/{}/{}".format(version, asset),
            "https://github.com/openai/kodex/releases/download/rust-v{}/{}".format(version, asset),
        ],
        sha256 = repository_ctx.attr.sha256,
    )
    manifest = json.decode(repository_ctx.read("kodex-package.json"))
    entrypoint = manifest["entrypoint"]
    binaries = entrypoint.rpartition("/")[0] + "/**"
    repository_ctx.file(
        "BUILD.bazel",
        _KODEX_RELEASE_BUILD_FILE.format(
            binaries = json.encode(binaries),
            entrypoint = json.encode(entrypoint),
            path = json.encode(manifest["pathDir"] + "/**"),
            resources = json.encode(manifest["resourcesDir"] + "/**"),
        ),
        executable = False,
    )
    return repository_ctx.repo_metadata(reproducible = True)

_kodex_release_repository = repository_rule(
    implementation = _kodex_release_repository_impl,
    attrs = {
        "sha256": attr.string(mandatory = True),
        "version": attr.string(mandatory = True),
    },
)

_RELEASE = tag_class(
    attrs = {
        "sha256": attr.string(mandatory = True),
        "version": attr.string(mandatory = True),
    },
)

def _kodex_release_archive_impl(module_ctx):
    for module in module_ctx.modules:
        for release in module.tags.release:
            _kodex_release_repository(
                name = "kodex_release_{}_linux_x86_64".format(release.version),
                sha256 = release.sha256,
                version = release.version,
            )

    return module_ctx.extension_metadata(reproducible = True)

kodex_release_archive = module_extension(
    implementation = _kodex_release_archive_impl,
    tag_classes = {"release": _RELEASE},
)
