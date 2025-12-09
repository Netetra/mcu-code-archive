module and_test;
  reg a, b;
  wire out;

  and(out, a, b);

  initial begin
    $dumpfile("and_test.vcd");
    $dumpvars(0, and_test);
    a = 0;
    b = 0;

    #10 a = 1;
    #10 a = 0; b = 1;
    #10 a = 1;
    #10 a = 0; b = 0;

    #10 $finish;
  end
endmodule
