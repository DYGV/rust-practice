module rawmode.file_io;

import std.stdio : File;
import rawmode.config : conf;
import std.stdio;
import std.conv;

void open_file(string file_name) {
    int i = 0;
    auto fin = File(file_name, "r");
    conf.file_name = file_name;
    foreach (line; fin.byLine) {
        conf.line_of_text.length += 1;
        conf.line_of_text[i++] = line.to!string;
        conf.numrows++;
    }
}
