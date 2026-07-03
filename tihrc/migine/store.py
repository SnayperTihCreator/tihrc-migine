from attrs import define, field

from .protocols import MathBlockProtocol, MigineProtocol


@define
class MigineStore:
    engine: MigineProtocol
    _blocks: dict[str, MathBlockProtocol] = field(factory=dict)
    
    def add_block(self, block: MathBlockProtocol):
        self._blocks[block.uid] = block
    
    def remove_block(self, block: MathBlockProtocol):
        return self._blocks.pop(block.uid)
    
    def get_blocks(self) -> list[MathBlockProtocol]:
        return list(self._blocks.values())
    
    def get_block_by_uid(self, uid: str) -> MathBlockProtocol:
        return self._blocks.get(uid, None)
