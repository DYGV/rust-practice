module rawmode.term;

import core.sys.posix.termios;
import core.sys.posix.sys.ioctl;
import core.sys.posix.unistd : STDIN_FILENO, STDOUT_FILENO;
import core.stdc.stdlib : atexit;
import rawmode.error : die;
import rawmode.config : conf;

void init() {
    // カーソル位置を管理する変数をそれぞれ0にする
    conf.cursor_x = 0;
    conf.cursor_y = 0;
    conf.numrows = 0;
    conf.rowoff = 0;
    conf.coloff = 0;
    conf.file_name = null;
    if (get_window_size(conf.screen_rows, conf.screen_cols) == -1) {
        "getWindowSize".die;
    }
    conf.screen_rows -= 2;
}

// 端末属性のリストアをする関数
// Cからから呼び出せるようにする
extern (C) void disable_raw_mode() {
    if (STDIN_FILENO.tcsetattr(TCSAFLUSH, &conf.orig_termios) == -1) {
        "tcsetattr".die;
    }
}

// Rawモードを有効にする関数
// ユーザからの行区切り文字の入力を待たずに端末から文字を読み取る(noncanonicalモード)
// noncanonicalモードでは、read()はどのような条件によりリターンするか考える 。
// 一定時間が経過した時点、もしくは一定のバイト数を読み取った時点でリターン する。(両方の時もある)
// termios構造体のc_cc配列内の2つの要素、TIMEおよびMINにより決定される。
// TIME(インデックスVTIME)は0.1秒単位でタイムアウト時間を表す。
// MIN(インデックスVMIN)は読み取るバイト数の最小値を表す。
void enable_raw_mode() {
    // 端末属性を取得する
    if (STDIN_FILENO.tcgetattr(&conf.orig_termios) == -1) {
        "tcgetattr".die;
    }
    // ハンドラとして端末属性を戻す関数を指定する
    (&disable_raw_mode).atexit;
    // バックアップ用
    termios raw = conf.orig_termios;
    // BRKINT: SIGINTを生成する(ctrl+Cの有効化)フラグ
    // ICRNL: キャリッジリターンをnewlineにマッピングするフラグ
    // INPCK: パリティーチェックの有効化フラグ
    // ISTRIP: 入力の8ビット目をクリアするフラグ
    // IXON: 出力フロー制御を有効にするフラグ
    // https://viewsourcecode.org/snaptoken/kilo/02.enteringRawMode.html#fix-ctrl-m
    // https://viewsourcecode.org/snaptoken/kilo/02.enteringRawMode.html#miscellaneous-flags
    raw.c_iflag &= ~(BRKINT | ICRNL | INPCK | ISTRIP | IXON);
    // OPOST: 出力の後処理を実行するフラグ
    // https://viewsourcecode.org/snaptoken/kilo/02.enteringRawMode.html#turn-off-all-output-processing
    // 出力機能処理を無効化する
    raw.c_oflag &= ~(OPOST);
    // CS8: 制御フラグ
    // https://viewsourcecode.org/snaptoken/kilo/02.enteringRawMode.html#miscellaneous-flags
    raw.c_cflag |= (CS8);
    // ECHO: 入力をエコーするフラグ
    // ICANON: canonicalモードの入力(行単位)フラグ
    // IEXTEN: 入力の拡張処理を有効にするフラグ
    // ISIG: シグナルを生成する文字を有効にする(INTR, QUIT, SUSP)フラグ
    // https://viewsourcecode.org/snaptoken/kilo/02.enteringRawMode.html#disable-ctrl-v
    raw.c_lflag &= ~(ECHO | ICANON | IEXTEN | ISIG);
    // タイムアウトを設定する。
    // read()実行時にタイマが開始され、
    // 少なくとも1バイト読み取った時点か、
    // 0.1秒経過した時点でリターンする。
    // https://viewsourcecode.org/snaptoken/kilo/02.enteringRawMode.html#a-timeout-for-read
    raw.c_cc[VMIN] = 0;
    raw.c_cc[VTIME] = 1;
    // TCSAFLUSH: 保留中の出力がすべて書き込まれるのを待ち、
    // キューイングされている入力を破棄する
    if (STDIN_FILENO.tcsetattr(TCSAFLUSH, &raw) == -1) {
        "tcsetattr".die;
    }
}

int get_window_size(ref int rows, ref int cols) {
    winsize ws;
    // wsに取得したウィンドウサイズを格納する
    if (ioctl(STDOUT_FILENO, TIOCGWINSZ, &ws) == -1 || ws.ws_col == 0) {
        return -1;
    } else {
        cols = ws.ws_col;
        rows = ws.ws_row;
        return 0;
    }
}
