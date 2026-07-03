from altgraph.Graph import Graph
from attrs import define, field

from .rendered import MigineRender
from .store import MigineStore
from .translator import MigineTranslator


@define
class Migine:
    renderer: MigineRender = field(init=False, repr=False)
    store: MigineStore = field(init=False, repr=False)
    translator: MigineTranslator = field(init=False, repr=False)
    _graph: Graph = field(factory=Graph, init=False, repr=False)
    
    def __attrs_post_init__(self):
        self.renderer = MigineRender(self)
        self.store = MigineStore(self)
        self.translator = MigineTranslator(self)