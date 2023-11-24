workspace(name = "everest-framework")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "rules_rust",
    sha256 = "36ab8f9facae745c9c9c1b33d225623d976e78f2cc3f729b7973d8c20934ab95",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.31.0/rules_rust-v0.31.0.tar.gz"],
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

rust_register_toolchains(
    versions = ["1.74.0"],
)

load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")

crate_universe_dependencies()

load("@rules_rust//crate_universe:defs.bzl", "crates_repository", "crate")

crates_repository(
    name = "crate_index",
    cargo_lockfile = "//everestrs:Cargo.lock",
    isolated = False,
    manifests = [
      "//everestrs:Cargo.toml",
      "//everestrs/everestrs:Cargo.toml",
      "//everestrs/everestrs-build:Cargo.toml",
    ],
)

load("@crate_index//:defs.bzl", "crate_repositories")

crate_repositories()

http_archive(
    name = "cxx.rs",
    sha256 = "3c591c1b6e8319bc055ec4a853f7eb7553e77caf9dd8cbec944b466eb3d06b76",
    urls = ["https://github.com/dtolnay/cxx/archive/refs/tags/1.0.110.tar.gz"],
    strip_prefix = "cxx-1.0.110"
)

load("@cxx.rs//third-party/bazel:defs.bzl", cxx_crate_repositories = "crate_repositories")

cxx_crate_repositories()

http_archive(
    name = "com_github_nelhage_rules_boost",
    url = "https://github.com/nelhage/rules_boost/archive/4ab574f9a84b42b1809978114a4664184716f4bf.tar.gz",
    sha256 = "2215e6910eb763a971b1f63f53c45c0f2b7607df38c96287666d94d954da8cdc",
    strip_prefix = "rules_boost-4ab574f9a84b42b1809978114a4664184716f4bf",
)
load("@com_github_nelhage_rules_boost//:boost/boost.bzl", "boost_deps")

boost_deps()

load("//third-party/bazel:deps.bzl", everest_framework_deps="everest_framework_deps")
everest_framework_deps()

load("@com_github_3rdparty_bazel_rules_curl//bazel:deps.bzl", curl_deps="deps")
curl_deps()
