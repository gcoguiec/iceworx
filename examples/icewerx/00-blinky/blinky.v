module blinky (
  input clk,
  output l1,
  output l2
);
  reg [31:0] counter = 32'h0;
  always @(posedge clk) begin
    counter <= counter + 1'b1;
  end
  assign l1 = counter[23];
  assign l2 = ~counter[23];
endmodule
