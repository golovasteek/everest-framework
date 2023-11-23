genrule(
  name = "compile_time_settings",
  outs = ["include/everest/compile_time_settings.hpp"],
  cmd = """
    echo "#define EVEREST_INSTALL_PREFIX \\"/usr\\"" > $@
    echo "#define EVEREST_NAMESPACE (\\"everest\\")" >> $@
  """,
)

cc_library(
  name = "framework",
  hdrs = glob(["include/**/*.hpp"]) + [":compile_time_settings"],
  srcs = glob(["lib/*.cpp"]),
  deps = [
    "@com_github_everest_liblog//:liblog",
    "@com_github_HowardHinnant_date//:date",
    "@com_github_nlohmann_json//:json",
    "@com_github_fmtlib_fmt//:fmt",
    "@com_github_biojppm_rapidyaml//:ryml",
    "@com_github_pboettch_json-schema-validator//:json-schema-validator",
    "@com_github_LiamBindle_mqtt-c//:libmqtt",
    "@//third-party/bazel:boost_uuid",
    "@//third-party/bazel:boost_program_options",
  ],
  strip_include_prefix = "include",
  copts = ["-std=c++17"],
  visibility = ["//visibility:public"],
)

cc_library(
  name = "controller-ipc",
  srcs = ["src/controller/ipc.cpp"],
  hdrs = ["src/controller/ipc.hpp"],
  deps = [
    "@com_github_nlohmann_json//:json",
  ],
  strip_include_prefix = "src",
  copts = ["-std=c++17"],
)

cc_binary(
  name = "controller",
  srcs = glob(
    ["src/controller/*.cpp", "src/controller/*.hpp"],
    exclude = ["src/controller/ipc.cpp", "src/controller/ipc.hpp"]
  ),
  deps = [
    ":controller-ipc",
    "@com_github_fmtlib_fmt//:fmt",
    "@com_github_biojppm_rapidyaml//:ryml",
    "@com_github_everest_liblog//:liblog",
    ":framework",
    "@com_github_warmcatt_libwebsockets//:libwebsockets",
  ],
  copts = ["-std=c++17"],
)

cc_binary(
  name = "manager",
  srcs = [
    "src/manager.cpp"
  ],
  deps = [
    "@com_github_everest_liblog//:liblog",
    "@com_github_fmtlib_fmt//:fmt",
    "@com_github_pboettch_json-schema-validator//:json-schema-validator",
    "@//third-party/bazel:boost_program_options",
    "@//third-party/bazel:boost_log",
    ":controller-ipc",
    ":framework",
  ],
  copts = ["-std=c++17"],
)

filegroup(
  name = "schemas",
  srcs = glob(["schemas/*.yaml"]),
  visibility = ["//visibility:public"],
)
