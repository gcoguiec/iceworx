package TestBench;

import Blinky::*;

(* synthesize *)
module mkTestBench(Empty);
  BlinkyInterface blinky <- mkBlinky;
  Reg#(Bit#(32)) cycle <- mkReg(0);
  Reg#(Bool) dump_started <- mkReg(False);
  let max_cycle = 10_000;

  rule cycle_incr;
    cycle <= cycle + 1;
    $display("cycle = %0h", cycle);
  endrule

  rule dump_start if (!dump_started);
    $dumpvars();
    dump_started <= True;
  endrule

  rule test if (cycle < max_cycle);
    $display("l1 = %0b", blinky.l1());
    $display("l2 = %0b", blinky.l2());
    if (cycle == max_cycle - 1) begin
      $finish(0);
    end
  endrule
endmodule: mkTestBench

endpackage: TestBench
