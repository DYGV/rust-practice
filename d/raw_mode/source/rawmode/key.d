module rawmode.key;

import std.stdio : writef;
import std.string : toStringz;
import core.sys.posix.unistd : STDIN_FILENO, STDOUT_FILENO, read, write;
import core.stdc.stdlib : exit;
import core.stdc.ctype : iscntrl;
import core.stdc.errno : errno, EAGAIN;
import rawmode.error : die;
import rawmode.screen : move_cursor;
import rawmode.config : conf;

enum Key {
    ARROW_LEFT = 1000,
    ARROW_RIGHT,
    ARROW_UP,
    ARROW_DOWN,
    DEL_KEY,
    HOME_KEY,
    END_KEY,
    PAGE_UP,
    PAGE_DOWN
};

// 入力の[a-z]を
// ctrl+[a-z]に変換する(下位5ビットを取り出す)
// 例: 'a'(0x61) & 0x1f == 1
//     'z'(0x7A) & 0x1f == 26
char ctrl_key(char k) {
    // 0001_1111で論理積
    return k & 0x1f;
}

// 標準入力から文字を読む関数
int read_key() {
    long nread;
    char c;
    while ((nread = STDIN_FILENO.read(&c, 1)) != 1) {
        if (nread == -1 && errno != EAGAIN) {
            "read".die;
        }
    }
    // エスケープシーケンスの処理
    // 矢印キー、del、PgUp、PgDn、Home、Endの処理
    // Home、Endは環境によって異なるので、それを網羅する
    // Home: <esc>[1~、<esc>[7~、<esc>[H、<esc>OH
    // End: <esc>[4~、<esc>[8~、<esc>[F、<esc>OF
    if (c == '\x1b') {
        char[3] seq;
        if (read(STDIN_FILENO, &seq[0], 1) != 1) {
            return '\x1b';
        }

        if (read(STDIN_FILENO, &seq[1], 1) != 1) {
            return '\x1b';
        }
        if (seq[0] == '[') {
            if (seq[1] >= '0' && seq[1] <= '9') {
                if (read(STDIN_FILENO, &seq[2], 1) != 1) {
                    return '\x1b';
                }
                if (seq[2] == '~') {
                    switch (seq[1]) {
                    case '1':
                        return Key.HOME_KEY;
                    case '2':
                        return Key.DEL_KEY;
                    case '4':
                        return Key.END_KEY;
                    case '5':
                        return Key.PAGE_UP;
                    case '6':
                        return Key.PAGE_DOWN;
                    case '7':
                        return Key.HOME_KEY;
                    case '8':
                        return Key.END_KEY;
                    default:
                        break;
                    }
                }
            } else {
                switch (seq[1]) {
                case 'A':
                    return Key.ARROW_UP;
                case 'B':
                    return Key.ARROW_DOWN;
                case 'C':
                    return Key.ARROW_RIGHT;
                case 'D':
                    return Key.ARROW_LEFT;
                case 'H':
                    return Key.HOME_KEY;
                case 'F':
                    return Key.END_KEY;
                default:
                    break;
                }
            }
        } else if (seq[0] == 'O') {
            switch (seq[1]) {
            case 'H':
                return Key.HOME_KEY;
            case 'F':
                return Key.END_KEY;
            default:
                break;
            }
        }
        return '\x1b';
    } else {
        return c;
    }
}

// 入力を読んで、その結果を処理する関数
void process_keypress() {
    int c = read_key();
    int times;
    switch (c) {
    case 'q'.ctrl_key:
        // 画面のクリア
        write(STDOUT_FILENO, "\x1b[2J".toStringz, 4);
        // カーソル位置を左上にする
        write(STDOUT_FILENO, "\x1b[H".toStringz, 3);
        exit(0);
        // Homeキーによるカーソル位置操作
    case Key.HOME_KEY:
        conf.cursor_x = 0;
        break;
        // Endキーによるカーソル位置操作
    case Key.END_KEY:
        conf.cursor_x = conf.screen_cols - 1;
        break;
        // PgUp、PgDnによるカーソル位置操作 
    case Key.PAGE_UP, Key.PAGE_DOWN:
        // 行数分、上もしくは下へ行く
        times = conf.screen_rows;
        while (times--) {
            move_cursor(c == Key.PAGE_UP ? Key.ARROW_UP : Key.ARROW_DOWN);
        }
        break;
        // 矢印キーによるカーソル位置操作
    case Key.ARROW_UP, Key.ARROW_LEFT, Key.ARROW_DOWN, Key.ARROW_RIGHT:
        move_cursor(c);
        break;
    default:
        break;
    }
}
