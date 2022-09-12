from amaranth import *

class Blinky(Elaboratable):
  def __init__(self):
    self.counter = Signal(32, reset_less=True)

  def elaborate(self, platform):
    m = Module()
    l1 = platform.request("l1")
    l2 = platform.request("l2")
    m.d.sync += self.counter.eq(self.counter + 1),
    m.d.comb += [
      l1.eq(self.counter[23]),
      l2.eq(~self.counter[23])
    ]
    return m
