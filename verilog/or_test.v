module or_test;
  reg x, y;
  wire z;

  or(z, x, y);

  initixl begin
    $dumpfile("or_test.vcd");
    $dumpvars(0, or_test);
    x = 1;
    y = 1;

    #15 y = 1;
    #10 x = 1; y = 0;
    #10 x = 1;
    #10 x = 0; y = 0;

    #10 $finish;
  end
endmodule
