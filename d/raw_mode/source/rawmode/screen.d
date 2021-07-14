module rawmode.screen;

import std.string : toStringz;
import core.sys.posix.unistd : STDOUT_FILENO, write;
import rawmode.config : conf;
import rawmode.key : Key;
import std.format : format;

void refresh_screen() {
    scroll();
    conf.buf = "";
    // ちらつき防止(カーソル非表示)
    conf.buf ~= "\x1b[?25l";
    // カーソル位置を左上にする
    conf.buf ~= "\x1b[H";
    // ~を描画する
    draw_rows();
    // ステータスバーの描画
    draw_status_bar();

    draw_message_bar();
    // カーソル位置をconf.cursorを基に変更する
    conf.buf ~= format!"\x1b[%d;%dH"(conf.cursor_y - conf.rowoff + 1,
            conf.cursor_x - conf.coloff + 1);
    // ちらつき防止(カーソル表示)
    conf.buf ~= "\x1b[?25h";
    write(STDOUT_FILENO, conf.buf.toStringz, conf.buf.length);
}

void set_status_message(string fmt) {
    conf.statusmsg = fmt;
}

void draw_message_bar() {
    conf.buf ~= "\x1b[K";
    conf.buf ~= conf.statusmsg;
}

void draw_status_bar() {
    conf.buf ~= "\x1b[7m";
    string status = format!"%.20s - %d lines"(conf.file_name
            ? conf.file_name : "[No Name]", conf.numrows);
    ulong len = status.length;
    string cur_status = format!"current line: %d/%d"(conf.cursor_y, conf.numrows);

    if (len > conf.screen_cols)
        len = conf.screen_cols;
    conf.buf ~= status;
    while (len < conf.screen_cols) {
        if (conf.screen_cols - len == cur_status.length) {
            conf.buf ~= cur_status;
            break;
        } else {
            conf.buf ~= " ";
            len++;
        }
    }
    conf.buf ~= "\x1b[m";
    conf.buf ~= "\r\n";
}

void draw_rows() {
    for (int y = 0; y < conf.screen_rows; y++) {
        int filerow = y + conf.rowoff;
        if (filerow >= conf.numrows) {
            // 行頭を表すチルダ
            conf.buf ~= "~";
        } else {
            ulong len = conf.line_of_text[filerow].length - conf.coloff;
            if (len < 0)
                len = 0;
            if (len > conf.screen_cols)
                len = conf.screen_cols;
            if (conf.coloff + len > conf.line_of_text[filerow].length) {
                conf.buf ~= conf.line_of_text[filerow];
            } else {
                conf.buf ~= conf.line_of_text[filerow][conf.coloff .. conf.coloff + len];
            }
        }
        // カーソルから行末まで消す
        conf.buf ~= "\x1b[K";
        conf.buf ~= "\r\n";
    }
}

void scroll() {
    if (conf.cursor_y < conf.rowoff) {
        conf.rowoff = conf.cursor_y;
    }
    if (conf.cursor_y >= conf.rowoff + conf.screen_rows) {
        conf.rowoff = conf.cursor_y - conf.screen_rows + 1;
    }
    if (conf.cursor_x < conf.coloff) {
        conf.coloff = conf.cursor_x;
    }
    if (conf.cursor_x >= conf.coloff + conf.screen_cols) {
        conf.coloff = conf.cursor_x - conf.screen_cols + 1;
    }

}

void move_cursor(int key) {
    string row = (conf.cursor_y >= conf.numrows) ? null : conf
        .line_of_text[conf.cursor_y];
    switch (key) {
    case Key.ARROW_LEFT: // 左へ
        // 負数にならないようにする
        if (conf.cursor_x != 0) {
            conf.cursor_x--;
        } else if (conf.cursor_y > 0) {
            // カーソル位置が左端のときに、左矢印キーが押下されたら
            // 上の行の末尾に移る
            conf.cursor_y--;
            conf.cursor_x = cast(int)conf.line_of_text[conf.cursor_y].length;
        }
        break;
    case Key.ARROW_RIGHT: // 右へ
        // その行の文字数を超えないようにする
        if (row && conf.cursor_x < row.length) {
            conf.cursor_x++;
        } else if (row && conf.cursor_x == row.length) {

            conf.cursor_y++;
            conf.cursor_x = 0;
        }
        break;
    case Key.ARROW_UP: // 上へ
        // 負数にならないようにする
        if (conf.cursor_y != 0) {
            conf.cursor_y--;
            // 上の行の文字数が現在のカーソルの位置より小さいとき
            // 上の行の末尾にカーソル位置を変える
            if (conf.line_of_text[conf.cursor_y].length < conf.cursor_x) {
                conf.cursor_x = cast(int)conf.line_of_text[conf.cursor_y].length;
            }
        }
        break;
    case Key.ARROW_DOWN: // 下へ
        if (conf.cursor_y < conf.numrows) {
            conf.cursor_y++;
        }
        break;
    default:
        break;
    }

    row = (conf.cursor_y >= conf.numrows) ? null : conf.line_of_text[conf
        .cursor_y];
    int rowlen = row ? cast(int)row.length : 0;
    if (conf.cursor_x > rowlen) {
        conf.cursor_x = rowlen;
    }
}
