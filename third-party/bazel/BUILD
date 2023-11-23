alias(
  name = "boost_program_options",
  actual = select({
    # "@//gorgeous:kirkstone": "@gorgeous_sysroot//:boost_program_options",
    "//conditions:default": "@boost//:program_options",
  }),
  visibility = ["//visibility:public"],
)

alias(
  name = "boost_log",
  actual = select({
    # "@//gorgeous:kirkstone": "@gorgeous_sysroot//:boost_log",
    "//conditions:default": "@boost//:log",
  }),
  visibility = ["//visibility:public"],
)


alias(
  name = "boost_current_function",
  actual = select({
    # "@//gorgeous:kirkstone": "@gorgeous_sysroot//:boost_current_function",
    "//conditions:default": "@boost//:current_function",
  }),
  visibility = ["//visibility:public"],
)
alias(
  name = "boost_uuid",
  actual = select({
    # "@//gorgeous:kirkstone": "@gorgeous_sysroot//:boost_uuid",
    "//conditions:default": "@boost//:uuid",
  }),
  visibility = ["//visibility:public"],
)
alias(
  name = "boost_exception",
  actual = select({
    # "@//gorgeous:kirkstone": "@gorgeous_sysroot//:boost_exception",
    "//conditions:default": "@boost//:exception",
  }),
  visibility = ["//visibility:public"],
)
alias (
  name = "curl",
  actual = select({
    # "@//gorgeous:kirkstone": "@gorgeous_sysroot//:curl",
    "//conditions:default": "@com_github_curl_curl//:libcurl_boringssl",
  }),
  visibility = ["//visibility:public"],
)

[
    alias(
        name = name,
        actual = "@crate_index__{}-{}//:{}".format(name, version, name),
        visibility = ["//visibility:public"],
    )
    for name, version in [
        ("anyhow", "1.0.75"),
        ("argh", "0.1.12"),
        ("argh_derive", "0.1.12"),
        ("argh_shared", "0.1.12"),
        ("cc", "1.0.83"),
        ("convert_case", "0.6.0"),
        ("cxx", "1.0.110"),
        ("cxxbridge-flags", "1.0.110"),
        ("cxxbridge-macro", "1.0.110"),
        ("equivalent", "1.0.1"),
        ("hashbrown", "0.14.2"),
        ("indexmap", "2.1.0"),
        ("itoa", "1.0.9"),
        ("libc", "0.2.150"),
        ("link-cplusplus", "1.0.9"),
        ("minijinja", "1.0.10"),
        ("proc-macro2", "1.0.69"),
        ("quote", "1.0.33"),
        ("ryu", "1.0.15"),
        ("serde", "1.0.193"),
        ("serde_derive", "1.0.193"),
        ("serde_json", "1.0.108"),
        ("serde_yaml", "0.9.27"),
        ("syn", "2.0.39"),
        ("thiserror", "1.0.50"),
        ("thiserror-impl", "1.0.50"),
        ("unicode-ident", "1.0.12"),
        ("unicode-segmentation", "1.10.1"),
        ("unsafe-libyaml", "0.2.9"),
    ]
]

