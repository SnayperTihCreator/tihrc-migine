from typing import Protocol, runtime_checkable

from .core import RuntimeContext


@runtime_checkable
class MathBlockProtocol(Protocol):
    @property
    def uid(self) -> str: ...
    
    @property
    def dirty(self) -> bool: ...
    
    @dirty.setter
    def dirty(self, value: bool) -> None: ...
    
    @property
    def isCached(self) -> bool: ...
    
    @property
    def dependencies(self) -> list["MathBlockProtocol"]: ...
    
    def execute(self, engine: "MigineProtocol", context: RuntimeContext) -> None: ...
    
    def restore(self, engine: "MigineProtocol", context: RuntimeContext) -> None: ...


@runtime_checkable
class MigineProtocol(Protocol):
    pass
