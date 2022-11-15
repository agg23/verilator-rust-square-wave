module top (
    /// Input at 100MHz
    input wire clk,

    output reg audio
);
  reg [31:0] counter = 0;

  always @(posedge clk) begin
    if (counter > 0) begin
      counter <= counter - 1;
    end else begin
      audio   <= ~audio;

      // 440Hz at 100MHz
      // Divide by two due to clock toggling
      counter <= 32'd100_000_000 / 32'd440 / 2;
    end
  end

endmodule
