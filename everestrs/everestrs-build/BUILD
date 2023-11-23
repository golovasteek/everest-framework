load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library")

filegroup(
  name = "templates",
  srcs = glob(["jinja/**/*"]),
)

rust_library(
  name = "everestrs-build",
  srcs = glob(["src/**/*.rs"], exclude = ["src/bin/**"]),
  deps = [
    "@//third-party/bazel:anyhow",
    "@//third-party/bazel:argh",
    "@//third-party/bazel:argh_shared",
    "@//third-party/bazel:convert_case",
    "@//third-party/bazel:minijinja",
    "@//third-party/bazel:serde",
    "@//third-party/bazel:serde_yaml",
  ],
  data = ["@everest-framework//everestrs/everestrs-build:templates"],
  visibility = ["//visibility:public"],
  edition = "2021",
)

rust_binary(
  name = "codegen",
  srcs = glob(["src/bin/**/*.rs"]),
  deps = [
    "@everest-framework//everestrs/everestrs-build:everestrs-build",
    "@//third-party/bazel:anyhow",
    "@//third-party/bazel:argh",
    "@//third-party/bazel:argh_shared",
    "@//third-party/bazel:convert_case",
    "@//third-party/bazel:minijinja",
    "@//third-party/bazel:serde",
    "@//third-party/bazel:serde_yaml",
  ],
  edition = "2021",
)
