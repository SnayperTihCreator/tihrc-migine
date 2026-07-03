from collections import UserDict
from typing import Any


class RuntimeContext(UserDict):
    def define(self, name: str, value: Any) -> None:
        self[name] = value
