package Blinky;

interface BlinkyInterface;
  method Bit#(1) l1();
  method Bit#(1) l2();
endinterface: BlinkyInterface

(* synthesize *)
(* clock_prefix = "clk" *)
(* always_enabled = "l1,l2" *)
(* no_default_reset *)
module mkBlinky(BlinkyInterface);
  Reg#(UInt#(12)) counter <- mkRegU();
  Reg#(Bit#(1)) l1_state <- mkRegU();
  Reg#(Bit#(1)) l2_state <- mkRegU();

  rule handle_cycle;
    counter <= counter + 1;
    l1_state <= pack(counter)[7];
    l2_state <= ~pack(counter)[7];
  endrule

  method Bit#(1) l1();
    return l1_state;
  endmethod

  method Bit#(1) l2();
    return l2_state;
  endmethod
endmodule: mkBlinky

endpackage: Blinky
