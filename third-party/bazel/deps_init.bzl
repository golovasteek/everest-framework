load("@cxx.rs//third-party/bazel:defs.bzl", cxx_crate_repositories = "crate_repositories")
load("@com_github_nelhage_rules_boost//:boost/boost.bzl", "boost_deps")
load("@com_github_3rdparty_bazel_rules_curl//bazel:deps.bzl", curl_deps="deps")
load("@everest-framework//third-party/bazel/rust:crates.bzl", crate_repositories="crate_repositories")


def deps_init():
    """Initializes the dependencies. Call `deps_fetch` first."""
    cxx_crate_repositories()
    boost_deps()
    curl_deps()
    crate_repositories()
