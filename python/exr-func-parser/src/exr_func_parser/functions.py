import logging
import re
from typing import Callable

from pydantic import BaseModel

log = logging.getLogger(__name__)


class Param(BaseModel):
    name: str
    type: str


class Func(BaseModel):
    name: str
    return_type: str
    declaration: str
    params: list[Param]

    def has_param_of_type(self, type_name) -> bool:
        return any(p for p in self.params if p.type == type_name)

    def has_param_name(self, name) -> bool:
        return any(p for p in self.params if p.name == name)


def normalize_declaration(func_decl: str) -> str:
    # Remove newlines
    func_decl = func_decl.replace("\n", " ")

    # Remove spaces after the (
    func_decl = re.sub(r"\(\s+", "(", func_decl)

    # Normalize spaces to a single consecutive one
    func_decl = re.sub(r"\s+", " ", func_decl)

    # Remove end semi-colon
    func_decl = func_decl.rstrip(";")

    return func_decl


def parse_param(param: str) -> Param:
    func_type, name = param.rsplit(maxsplit=1)
    return Param(name=name.strip(), type=func_type.strip())


def parse_params_list(func_decl) -> list[Param]:
    params = []
    # Find everything between (...)
    for params_list in re.findall(r"\(([^)]*)\)", func_decl):
        for param in params_list.split(","):
            params.append(parse_param(param))
    return params


def parse_func_info(func_decl: str) -> tuple[str, str]:
    info, _ = func_decl.split("(", maxsplit=1)
    return_type, name = info.strip().rsplit(maxsplit=1)
    return name, return_type


def parse_function(func_decl: str) -> Func:
    func_decl = normalize_declaration(func_decl)

    params = parse_params_list(func_decl)

    name, return_type = parse_func_info(func_decl)

    return Func(
        name=name,
        return_type=return_type,
        params=params,
        declaration=func_decl,
    )


def filter_funcs(
    files_funcs: dict[str, list[Func]],
    func_filter: Callable[[Func], bool],
) -> dict[str, list[Func]]:
    filtered_funcs = {}
    for file_name, funcs in files_funcs.items():
        if filter_res := [f for f in funcs if func_filter(f)]:
            filtered_funcs[file_name] = filter_res
    return filtered_funcs


def split_funcs(
    files_funcs: dict[str, list[Func]],
    func_filter: Callable[[Func], bool],
) -> tuple[dict[str, list[Func]], dict[str, list[Func]]]:
    match_funcs = {}
    non_match_funcs = {}
    for file_name, funcs in files_funcs.items():
        match_res, non_match_res = [], []
        for f in funcs:
            if func_filter(f):
                match_res.append(f)
            else:
                non_match_res.append(f)

        if match_res:
            match_funcs[file_name] = match_res
        if non_match_res:
            non_match_funcs[file_name] = non_match_res

    return match_funcs, non_match_funcs
