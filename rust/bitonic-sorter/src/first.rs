// publicな関数として他のモジュールからアクセスできることを示す
// xの引数の型 = &はポインタ経由で借用することを示す
// mutはmutable
// u32は32bit unsigned int
// [u32]型はu32のスライス(1次元配列)

// Rustでは関数、変数、定数にスネークケース、
// ユーザが定義した型やジェネリクス型パラメータの識別子にはキャメルケースを用いる
pub fn sort(x: &mut [u32], up: bool) {
    // スライスの参照を受け取っているため値を返す必要がない
    if x.len() > 1 {
        let mid_point = x.len() / 2;
        sort(&mut x[..mid_point], true);
        sort(&mut x[mid_point..], false);
        sub_sort(x, up)
    }
}

fn sub_sort(x: &mut [u32], up: bool) {
    if x.len() > 1 {
        compare_and_swap(x, up);
        let mid_point = x.len() / 2;
        sub_sort(&mut x[..mid_point], up);
        sub_sort(&mut x[mid_point..], up);
    }
}

fn compare_and_swap(x: &mut [u32], up: bool) {
    let mid_point = x.len() / 2;
    for i in 0..mid_point {
        if (x[i] > x[mid_point + i]) == up {
            // 2つの要素を交換するswapメソッドを使う
            x.swap(i, mid_point + i)
        }
    }
}

// このモジュールはcargo testを実行したときのみコンパイルされる
#[cfg(test)]
mod tests {
    // 親モジュール(first)のsort関数を使用する
    use super::sort;

    // テストケースになる関数は#[test]アトリビュートを付ける
    #[test]
    fn sort_u32_ascending() {
        // テストデータとしてu32型のベクタを作成しxに束縛する
        // sort関数によってxの内容は更新されるのでmutableに
        let mut x = vec![10, 30, 11, 20, 4, 330, 21, 110];

        // xのスライスを作成し、sort関数を呼び出す
        // &mut xは&mut x[..]と書いてもよい
        sort(&mut x, true);
        assert_eq!(x, vec![4, 10, 11, 20, 21, 30, 110, 330]);
    }

    #[test]
    fn sort_u32_descending() {
        let mut x = vec![10, 30, 11, 20, 4, 330, 21, 110];
        sort(&mut x, false);
        assert_eq!(x, vec![330, 110, 30, 21, 20, 11, 10, 4]);
    }
}
