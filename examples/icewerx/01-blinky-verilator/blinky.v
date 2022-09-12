module blinky (
  input clk,
  output l1,
  output l2
);
  reg [11:0] counter = 12'h0;
  always @(posedge clk) begin
    counter <= counter + 1'b1;
  end
  assign l1 = counter[7];
  assign l2 = ~counter[7];
endmodule
