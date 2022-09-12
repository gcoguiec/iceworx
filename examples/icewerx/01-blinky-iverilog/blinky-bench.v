`timescale 10us/10us

module blinky_bench();
  reg clk = 1'b0;
  wire l1;
  wire l2;

  blinky blinky (
    .clk(clk),
    .l1(l1),
    .l2(l2)
  );

  integer idx;

  initial begin
    $dumpfile("blinky.lxt");
    $dumpvars(0, blinky_bench);
    for (idx = 0 ;  idx <= 10_000 ;  idx = idx + 1) begin
      #1 clk = ~clk;
      $display("l1 = %b", l1);
      $display("l2 = %b", l2);
    end
    $finish;
  end
endmodule
