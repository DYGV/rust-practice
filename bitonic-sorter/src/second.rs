// Ordトレイトを実装した型ならなんでもソートできるようなプログラム

// publicな関数として他のモジュールからアクセスできることを示す
// xの引数の型 = &はポインタ経由で借用することを示す
// mutはmutable
// u32は32bit unsigned int
// [u32]型はu32のスライス(1次元配列)

// Rustでは関数、変数、定数にスネークケース、
// ユーザが定義した型やジェネリクス型パラメータの識別子にはキャメルケースを用いる

use super::SortOrder;

// match式による場合分けをしてdo_sort()に渡す
pub fn sort<T: Ord>(x: &mut [T], order: &SortOrder) -> Result<(), String> {
    if x.len().is_power_of_two() {
        match *order {
            SortOrder::Ascending => do_sort(x, true),
            SortOrder::Descending => do_sort(x, false),
        };
        Ok(())
    } else {
        Err(format!(
            "The length of x is not a power of two. (x.len(): {})",
            x.len()
        ))
    }
}

// u32型のみに対応している
// pub fn sort(x: &mut [u32], up: bool) {
// 型パラメータTを導入して、関数をジェネリクス化する
// 全順序だけを受け取るように、型パラメータTのトレイト境界としてOrdを設定
fn do_sort<T: Ord>(x: &mut [T], up: bool) {
    // スライスの参照を受け取っているため値を返す必要がない
    if x.len() > 1 {
        let mid_point = x.len() / 2;
        do_sort(&mut x[..mid_point], true);
        do_sort(&mut x[mid_point..], false);
        sub_sort(x, up)
    }
}

fn sub_sort<T: Ord>(x: &mut [T], up: bool) {
    if x.len() > 1 {
        compare_and_swap(x, up);
        let mid_point = x.len() / 2;
        sub_sort(&mut x[..mid_point], up);
        sub_sort(&mut x[mid_point..], up);
    }
}

fn compare_and_swap<T: Ord>(x: &mut [T], up: bool) {
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
    use crate::SortOrder::*;

    // テストケースになる関数は#[test]アトリビュートを付ける
    #[test]
    fn sort_u32_ascending() {
        // テストデータとしてu32型のベクタを作成しxに束縛する
        // sort関数によってxの内容は更新されるのでmutableに

        // xに型注釈Vec<u32>を付ける
        // firstでは型推論をしていた
        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];

        // xのスライスを作成し、sort関数を呼び出す
        // &mut xは&mut x[..]と書いてもよい
        assert_eq!(sort(&mut x, &Ascending), Ok(()));
        assert_eq!(x, vec![4, 10, 11, 20, 21, 30, 110, 330]);
    }

    #[test]
    fn sort_u32_descending() {
        // xに型注釈Vec<u32>を付ける
        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];
        assert_eq!(sort(&mut x, &Descending), Ok(()));
        assert_eq!(x, vec![330, 110, 30, 21, 20, 11, 10, 4]);
    }

    #[test]
    fn sort_str_ascending() {
        // 文字列のベクタを作り、ソートする
        let mut x = vec![
            "Rust",
            "is",
            "fast",
            "and",
            "memory-efficient",
            "with",
            "no",
            "GC",
        ];
        assert_eq!(sort(&mut x, &Ascending), Ok(()));
        assert_eq!(
            x,
            vec![
                "GC",
                "Rust",
                "and",
                "fast",
                "is",
                "memory-efficient",
                "no",
                "with"
            ]
        );
    }

    #[test]
    fn sort_str_descending() {
        let mut x = vec![
            "Rust",
            "is",
            "a",
            "system",
            "programming",
            "language",
            "that",
            "runs",
        ];
        assert_eq!(sort(&mut x, &Descending), Ok(()));
        assert_eq!(
            x,
            vec![
                "that",
                "system",
                "runs",
                "programming",
                "language",
                "is",
                "a",
                "Rust"
            ]
        );
    }

    #[test]
    fn sort_to_fail() {
        let mut x = vec![10, 30, 11]; // 2のべき乗でない
        assert!(sort(&mut x, &Ascending).is_err()); // 戻り値はErr
    }
}
