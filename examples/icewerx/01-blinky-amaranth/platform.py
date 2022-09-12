from amaranth.build import *
from amaranth.vendor.lattice_ice40 import *

__all__ = ["IceWerxPlatform"]

class IceWerxPlatform(LatticeICE40Platform):
  device = "iCE40HX8K"
  package = "CB132"
  default_clk = "clk"
  resources = [
    Resource("clk", 0, Pins("P7", dir="i"), Clock(12e6), Attrs(GLOBAL=True, IO_STANDARD="SB_LVCMOS")),
    Resource("l1", 0, Pins("A5", dir="o"), Attrs(IO_STANDARD="SB_LVCMOS")),
    Resource("l2", 0, Pins("M4", dir="o"), Attrs(IO_STANDARD="SB_LVCMOS"))
  ]
  connectors = []

  def toolchain_program(self, products, name):
    # This platform could be interfaced with pitrz/icefunprog (in Python) instead of iceworx.
    return
