module rawmode.config;

import core.sys.posix.termios : termios;

struct Config {
    string buf;
    string[] line_of_text;
    int rowoff, coloff;
    int numrows;
    int screen_rows, screen_cols;
    int cursor_x, cursor_y;
    string file_name;
    string statusmsg;
    termios orig_termios;
};

Config conf;
