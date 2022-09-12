from amaranth.sim import Simulator

from blinky import *

blinky = Blinky()

def testbench():
  yield

sim = Simulator(blinky) # Simulation can't be used with Platforms (yet), this will fail.
sim.add_clock(12e6)
sim.add_sync_process(testbench)
with sim.write_vcd("blinky.vcd"):
  sim.run()
