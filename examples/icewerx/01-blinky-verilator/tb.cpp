#include "Vblinky.h"
#include <iostream>
#include <stdio.h>
#include <verilated.h>
#include <verilated_vcd_c.h>

using namespace std;

#define MAX_SIM_TIME 300000

int main(int argc, char **argv, char **env) {
  const std::unique_ptr<VerilatedContext> contextp{new VerilatedContext};
  Verilated::commandArgs(argc, argv);
  Verilated::traceEverOn(true);

  Vblinky *blinky = new Vblinky;
  VerilatedVcdC *vcd = new VerilatedVcdC;

  blinky->trace(vcd, 10);
  vcd->open("obj_dir/blinky.vcd");

  while (contextp->time() < MAX_SIM_TIME && !contextp->gotFinish()) {
    contextp->timeInc(1);
    blinky->clk ^= 1;
    blinky->eval();
    vcd->dump(contextp->time());
  }
  blinky->final();
  vcd->close();

  delete blinky;
  delete vcd;

  exit(EXIT_SUCCESS);
}
