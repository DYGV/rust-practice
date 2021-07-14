import std.getopt : getopt;
import rawmode;

void main(string[] args) {
    string file_name;
    getopt(args, "filename", &file_name);
    rawmode.enable_raw_mode();
    rawmode.init();
    if(file_name.length > 0){
        rawmode.open_file(file_name);
    }
    rawmode.set_status_message("HELP: Ctrl-Q = quit");
    while (1) {
        rawmode.refresh_screen();
        rawmode.process_keypress();
    }
}
