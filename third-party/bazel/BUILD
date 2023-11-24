[
    alias(
        name = "boost_{}".format(name),
        actual = "@boost//:{}".format(name),
        visibility = ["//visibility:public"],
    )
    for name in [
        "program_options",
        "log",
        "current_function",
        "uuid",
        "exception",
    ]
]

alias (
  name = "curl",
  actual = "@com_github_curl_curl//:libcurl_boringssl",
  visibility = ["//visibility:public"],
)

[
    alias(
        name = name,
        actual = "//third-party/bazel/rust:{}".format(name),
        visibility = ["//visibility:public"],
    )
    for name in [
        "anyhow",
        "argh",
        "convert_case",
        "cxx",
        "minijinja",
        "serde",
        "serde_json",
        "serde_yaml",
        "thiserror",
    ]
]
