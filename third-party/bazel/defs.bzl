load("@rules_foreign_cc//foreign_cc:repositories.bzl", "rules_foreign_cc_dependencies")
load("@com_github_nelhage_rules_boost//:boost/boost.bzl", "boost_deps")
load("@com_github_everest_everest-framework//third-party/bazel/rust:crates.bzl", "crate_repositories")
load("@cxx.rs//third-party/bazel:defs.bzl", cxx_crate_repositories = "crate_repositories")

def everest_framework_deps(repo_mapping = {}):
    cxx_crate_repositories()
    boost_deps()
    rules_foreign_cc_dependencies()
    crate_repositories()
