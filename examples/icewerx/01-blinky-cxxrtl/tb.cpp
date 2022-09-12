#include <fstream>
#include <iostream>

#include <backends/cxxrtl/cxxrtl_vcd.h>

#include "blinky.cpp"

using namespace std;

void dump_item_value(cxxrtl::debug_items &items, std::string path) {
  cxxrtl::debug_item item = items.at(path);

  const size_t nr_chunks =
      (item.width + (sizeof(chunk_t) * 8 - 1)) / (sizeof(chunk_t) * 8);

  cout << "\"" << path << "\":" << endl;

  for (size_t index = 0; index < item.depth; index++) {
    if (item.depth > 1)
      cout << "location[" << index << "] : ";

    for (size_t chunk_nr = 0; chunk_nr < nr_chunks; ++chunk_nr) {
      cout << item.curr[nr_chunks * index + chunk_nr];
      cout << (chunk_nr == nr_chunks - 1 ? "\n" : ",  ");
    }
  }
}

int main(int argc, char **argv) {
  cxxrtl_design::p_blinky blinky;
  cxxrtl::debug_items all_debug_items;
  blinky.debug_info(all_debug_items);
  cxxrtl::vcd_writer vcd;
  vcd.timescale(1, "us");
  vcd.add_without_memories(all_debug_items);

  std::ofstream waves("blinky.vcd");

  blinky.step();
  vcd.sample(0);
  for (int cycle = 0; cycle < 10000; ++cycle) {
    blinky.p_clk.set<bool>(false);
    blinky.step();
    vcd.sample(cycle * 2);

    blinky.p_clk.set<bool>(true);
    blinky.step();
    vcd.sample(cycle * 2 + 1);

    dump_item_value(all_debug_items, "counter");
    dump_item_value(all_debug_items, "l1");
    dump_item_value(all_debug_items, "l2");
    dump_item_value(all_debug_items, "clk");

    waves << vcd.buffer;
    vcd.buffer.clear();
  }

  exit(EXIT_SUCCESS);
}
