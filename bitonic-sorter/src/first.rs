// publicな関数として他のモジュールからアクセスできることを示す
// xの引数の型 = &はポインタ経由で借用することを示す
// mutはmutable
// u32は32bit unsigned int
// [u32]型はu32のスライス(1次元配列)

// Rustでは関数、変数、定数にスネークケース、
// ユーザが定義した型やジェネリクス型パラメータの識別子にはキャメルケースを用いる
pub fn sort(x: &mut [u32], up: bool) {
    unimplemented!();
}

fn sub_sort(x: &mut [u32], up: bool) {
    unimplemented!();
}

fn compare_and_swap(x: &mut [u32], up: bool) {
    unimplemented!();
}
