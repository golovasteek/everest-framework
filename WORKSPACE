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
    # When you first run this tool, it'll recommend a sha256 hash to put here with a message like: "DEBUG: Rule 'com_github_nelhage_rules_boost' indicated that a canonical reproducible form can be obtained by modifying arguments sha256 = ..."
)
load("@com_github_nelhage_rules_boost//:boost/boost.bzl", "boost_deps")

boost_deps()

http_archive(
  name = "com_github_everest_liblog",
  url = "https://github.com/EVerest/liblog/archive/5c132fb0bcdfb41d6285c4f834ae4657d7a6bff6.tar.gz",
  sha256 = "19b017a8ef4948bd0692f1d683d0031e76daa21d43a4bddff818bb77e0827fac",
  strip_prefix = "liblog-5c132fb0bcdfb41d6285c4f834ae4657d7a6bff6",
  build_file = "//third-party/bazel:BUILD.liblog.bazel",
)

http_archive(
  name = "com_github_pboettch_json-schema-validator",
  url = "https://github.com/pboettch/json-schema-validator/archive/f4194d7e24e2e2365660ff35b57a7c4e088b27fa.tar.gz",
  sha256 = "f71f2fbef135a61ad7bd9444f4202f9698a4b1c70279cb1e9b2567a6d996aaa1",
  strip_prefix = "json-schema-validator-f4194d7e24e2e2365660ff35b57a7c4e088b27fa",
  build_file = "//third-party/bazel:BUILD.json-schema-validator.bazel",
)

http_archive(
  name = "com_github_HowardHinnant_date",
  url = "https://github.com/HowardHinnant/date/archive/2ef74cb41a31dcd03b39dd5e9bf8b340669f48a4.tar.gz",
  sha256 = "3446ad7e2ba07c9105769bf6fd9b521d4e3a2f2dd0a955643a20f4adb1870efa",
  strip_prefix = "date-2ef74cb41a31dcd03b39dd5e9bf8b340669f48a4",
  build_file = "//third-party/bazel:BUILD.date.bazel",
)

http_archive(
  name = "com_github_biojppm_rapidyaml",
  url = "https://github.com/biojppm/rapidyaml/archive/213b201d264139cd1b887790197e08850af628e3.tar.gz",
  sha256 = "c206d4565ccfa721991a8df90821d1a1f747e68385a0f3f5b9ab995e191c06be",
  strip_prefix = "rapidyaml-213b201d264139cd1b887790197e08850af628e3",
  build_file = "//third-party/bazel:BUILD.rapidyaml.bazel",
)

http_archive(
  name = "com_github_biojppm_c4core",
  url = "https://github.com/biojppm/c4core/archive/d35c7c9bf370134595699d791e6ff8db018ddc8d.tar.gz",
  sha256 = "b768c8fb5dd4740317b7e1a3e43a0b32615d4d4e1e974d7ab515a80d2f1f318d",
  strip_prefix = "c4core-d35c7c9bf370134595699d791e6ff8db018ddc8d",
  build_file = "//third-party/bazel:BUILD.c4core.bazel",
)

http_archive(
  name = "com_github_biojppm_debugbreak",
  url = "https://github.com/biojppm/debugbreak/archive/5dcbe41d2bd4712c8014aa7e843723ad7b40fd74.tar.gz",
  sha256 = "4b424d23dad957937c57c142648d32571a78a6c6b2e473709748e5c1bb8a0f92",
  strip_prefix = "debugbreak-5dcbe41d2bd4712c8014aa7e843723ad7b40fd74",
  build_file = "//third-party/bazel:BUILD.debugbreak.bazel",
)

http_archive(
  name = "com_github_fastfloat_fastfloat",
  url = "https://github.com/fastfloat/fast_float/archive/32d21dcecb404514f94fb58660b8029a4673c2c1.tar.gz",
  sha256 = "8035a9ca28a8e3dfee332c7960af3c06ef0ab5169d5f31228b89c469e882bef7",
  strip_prefix = "fast_float-32d21dcecb404514f94fb58660b8029a4673c2c1",
  build_file = "//third-party/bazel:BUILD.fastfloat.bazel",
)

load("//third-party/bazel/3rdparty/bazel-rules-curl:repos.bzl", curl_repos="repos")
curl_repos()

load("@com_github_3rdparty_bazel_rules_curl//bazel:deps.bzl", curl_deps="deps")
curl_deps()

http_archive(
  name = "com_github_warmcatt_libwebsockets",
  url = "https://github.com/warmcat/libwebsockets/archive/b0a749c8e7a8294b68581ce4feac0e55045eb00b.tar.gz",
  sha256 = "5c3d6d482e73a0c105f3f14ce9b03c27b5d51c3938f8483b7eb8e6d535baa25f",
  strip_prefix = "libwebsockets-b0a749c8e7a8294b68581ce4feac0e55045eb00b",
  build_file = "//third-party/bazel:BUILD.libwebsockets.bazel",
)

http_archive(
  name = "com_github_LiamBindle_mqtt-c",
  url = "https://github.com/LiamBindle/MQTT-C/archive/f69ce1e7fd54f3b1834c9c9137ce0ec5d703cb4d.tar.gz",
  sha256 = "0b3ab84e5bca3c0c29be6b84af6f9840d92a0ae4fc00ca74fdcacc30b2b0a1e9",
  strip_prefix = "MQTT-C-f69ce1e7fd54f3b1834c9c9137ce0ec5d703cb4d",
  build_file = "//third-party/bazel:BUILD.mqtt-c.bazel",
)

http_archive(
    name = "com_github_everest_everest-utils",
    urls = ["https://github.com/golovasteek/everest-utils/archive/1fbe4aa1da659c7359030cba0ff15e1b9ba99052.tar.gz"],
    strip_prefix = "everest-utils-1fbe4aa1da659c7359030cba0ff15e1b9ba99052",
    build_file = "//everest/everest-utils:BUILD.everest-utils.bazel"
)

http_archive(
    name = "com_github_fmtlib_fmt",
    urls = ["https://github.com/fmtlib/fmt/archive/refs/tags/10.1.1.tar.gz"],
    strip_prefix = "fmt-10.1.1",
    sha256 = "78b8c0a72b1c35e4443a7e308df52498252d1cefc2b08c9a97bc9ee6cfe61f8b",
    build_file = "//third-party/bazel:BUILD.fmt.bazel",
)

http_archive(
    name = "com_github_nlohmann_json",
    urls = ["https://github.com/nlohmann/json/archive/refs/tags/v3.11.2.tar.gz"],
    strip_prefix = "json-3.11.2",
    sha256 = "d69f9deb6a75e2580465c6c4c5111b89c4dc2fa94e3a85fcd2ffcd9a143d9273",
    build_file = "//third-party/bazel:BUILD.nlohmann_json.bazel",
)
