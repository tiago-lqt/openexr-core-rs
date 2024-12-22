import logging
from pathlib import Path
from typing import Callable
import exr_func_parser
from exr_func_parser.functions import filter_funcs, split_funcs

logging.basicConfig(
    level=logging.DEBUG, format="%(asctime)s - %(levelname)s - %(message)s"
)

log = logging.getLogger(__name__)

OPENEXR_FILES = [
    "openexr_attr.h",
    "openexr_base.h",
    "openexr_chunkio.h",
    "openexr_coding.h",
    "openexr_compression.h",
    "openexr_config.h",
    "openexr_context.h",
    "openexr_debug.h",
    "openexr_decode.h",
    "openexr_encode.h",
    "openexr_errors.h",
    "openexr_part.h",
    "openexr_std_attr.h",
]

root_dir = (
    Path(f"{__file__}").parent.joinpath("../vendor/openexr/include/OpenEXR").resolve()
)

files: list[exr_func_parser.File] = []
for file in OPENEXR_FILES:
    res = exr_func_parser.parse(root_dir.joinpath(file))
    if res.functions:
        log.info("Parsed %s functions", len(res.functions))
        files.append(res)


def print_funcs(files_funcs: dict[str, list[exr_func_parser.Func]]) -> None:
    for file_name, funcs in files_funcs.items():
        print(file_name)
        for func in funcs:
            print(func.declaration)
        print("")


file_funcs = {file.name: file.functions for file in files}

# Find all funcs not requiring read/write contexts
static_funcs, non_static_funcs = split_funcs(
    file_funcs,
    lambda f: not (
        f.has_param_of_type("exr_const_context_t")
        or f.has_param_of_type("exr_context_t")
        or f.has_param_of_type("exr_context_t*")
    ),
)
# print_funcs(static_funcs)

# Find all funcs using const context
const_ctx_funcs, ctx_funcs = split_funcs(
    non_static_funcs, lambda f: f.has_param_of_type("exr_const_context_t")
)
print_funcs(ctx_funcs)

# part_funcs = filter_funcs(const_ctx_funcs, lambda f: not f.has_param_name("part_index"))
# print_funcs(part_funcs)


# # Find all funcs using encode pipeline
# encode_pipeline_funcs = filter_funcs(
#     file_funcs, lambda f: f.has_param_of_type("exr_encode_pipeline_t*")
# )
# print_funcs(encode_pipeline_funcs)

pass
