module rawmode.error;

import core.stdc.stdio : perror;
import core.stdc.stdlib : exit;
import std.string : toStringz;
import core.sys.posix.unistd : STDOUT_FILENO, write;

void die(const char* s) {
    // 画面のクリア
    write(STDOUT_FILENO, "\x1b[2J".toStringz, 4);
    // カーソル位置を左上にする
    write(STDOUT_FILENO, "\x1b[H".toStringz, 3);
    // sを標準エラー出力へ出力する
    perror(s);
    // 異常終了
    exit(1);
}
