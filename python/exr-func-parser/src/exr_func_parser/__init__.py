import logging
from pathlib import Path
import re

from exr_func_parser.functions import Func, parse_function
from pydantic import BaseModel

log = logging.getLogger(__name__)


class File(BaseModel):
    name: str
    functions: list[Func]


def parse(path: Path) -> File:
    log.info("Parsing %s", path)

    content = path.read_text()

    # Match all functions in the file
    pattern = r"\b[A-Za-z_][A-Za-z0-9_]*\s+\**\b[A-Za-z_][A-Za-z0-9_]*\s*\([^)]*\)\s*;"
    matches = re.findall(pattern, content)

    functions = []
    for match in matches:
        func = parse_function(match)
        functions.append(func)

    file = File(name=path.name, functions=functions)
    return file
