// Ordトレイトを実装した型ならなんでもソートできるようなプログラム

// publicな関数として他のモジュールからアクセスできることを示す
// xの引数の型 = &はポインタ経由で借用することを示す
// mutはmutable
// u32は32bit unsigned int
// [u32]型はu32のスライス(1次元配列)

// Rustでは関数、変数、定数にスネークケース、
// ユーザが定義した型やジェネリクス型パラメータの識別子にはキャメルケースを用いる
use super::SortOrder;
use rayon;
use std::cmp::Ordering;

// match式による場合分けをしてdo_sort()に渡す
pub fn sort<T: Ord + Send>(x: &mut [T], order: &SortOrder) -> Result<(), String> {
    // do_sort()を呼ぶ代わりに、sort_by()を呼ぶようにする

    match *order {
        // 昇順ならa.cmp(b), 降順ならb.cmp(a)を行う
        SortOrder::Ascending => sort_by(x, &|a, b| a.cmp(b)),
        SortOrder::Descending => sort_by(x, &|a, b| b.cmp(a)),
    }
}

// 第2引数comparatorはクロージャを受け取る
// クロージャの型はジェネリクスになっていて、型パラメータFで示している
// where節以降にはFnで始まるトレイト境界が指定されている
pub fn sort_by<T, F>(x: &mut [T], comparator: &F) -> Result<(), String>
where
    T: Send,
    F: Sync + Fn(&T, &T) -> Ordering,
{
    if x.len().is_power_of_two() {
        do_sort(x, true, comparator);
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

const PARALLEL_THRESHOLD: usize = 4096;

fn do_sort<T, F>(x: &mut [T], forward: bool, comparator: &F)
where
    T: Send,
    F: Sync + Fn(&T, &T) -> Ordering,
{
    if x.len() > 1 {
        let mid_point = x.len() / 2;
        // xをmid_pointを境にした2つの可変の借用に分割し
        // firstとsecondに束縛する
        let (first, second) = x.split_at_mut(mid_point);
        // xの分割後の閾値と比較する
        if mid_point >= PARALLEL_THRESHOLD {
            // 閾値以上なら並列にソートする
            rayon::join(
                || do_sort(first, true, comparator),
                || do_sort(second, false, comparator),
            );
        } else {
            // 閾値未満なら順番にソートする
            // 第2引数がtrueのときはcomparatorで示される順にソート
            do_sort(&mut x[..mid_point], true, comparator);
            // 第2引数がfalseのときはcomparatorとは逆順にソート
            do_sort(&mut x[mid_point..], false, comparator);
        }
        sub_sort(x, forward, comparator)
    }
}

fn sub_sort<T, F>(x: &mut [T], forward: bool, comparator: &F)
where
    T: Send,
    F: Sync + Fn(&T, &T) -> Ordering,
{
    // 受け取ったforward引数をcompare_and_swap関数や自分自身の再帰呼び出しにそのまま渡す
    if x.len() > 1 {
        compare_and_swap(x, forward, comparator);
        let mid_point = x.len() / 2;
        let (first, second) = x.split_at_mut(mid_point);
        // xの分割後の閾値と比較する
        if mid_point >= PARALLEL_THRESHOLD {
            // 閾値以上なら並列にソートする
            rayon::join(
                || sub_sort(first, forward, comparator),
                || sub_sort(second, forward, comparator),
            );
        } else {
            // 閾値未満なら順番にソートする
            // 第2引数がtrueのときはcomparatorで示される順にソート
            sub_sort(first, forward, comparator);
            // 第2引数がfalseのときはcomparatorとは逆順にソート
            sub_sort(second, forward, comparator);
        }
    }
}

// 2つの要素の比較にcomparatorクロージャを使う
fn compare_and_swap<T, F>(x: &mut [T], forward: bool, comparator: &F)
where
    F: Fn(&T, &T) -> Ordering,
{
    // 比較に先立ちforward(bool値)をOrdering値に変換しておく
    let swap_condition = if forward {
        Ordering::Greater
    } else {
        Ordering::Less
    };
    let mid_point = x.len() / 2;
    for i in 0..mid_point {
        // comparatorクロージャで2要素を比較し、返されたOrderingのバリアントが
        // swap_conditionと等しいなら要素を交換する
        // comparatorクロージャはGreater, Equal, Lessのいずれかのバリアントを返す
        // この値とforward(bool型)は比較できないため、上でOrdering型へ変換した
        if comparator(&x[i], &x[mid_point + i]) == swap_condition {
            // 2つの要素を交換するswapメソッドを使う
            x.swap(i, mid_point + i)
        }
    }
}

// このモジュールはcargo testを実行したときのみコンパイルされる
#[cfg(test)]
mod tests {
    // 親モジュール(first)のsort関数を使用する
    use super::{sort, sort_by};
    use crate::utils::{is_sorted_ascending, is_sorted_descending, new_u32_vec};
    use crate::SortOrder::*;

    // deriveアトリビュートを使い、DebugトレイトとPartialEqトレイトの実装を自動導出する
    #[derive(Debug, PartialEq)]
    struct Student {
        first_name: String, // String型の名前フィールド
        last_name: String,  // String型の苗字フィールド
        age: u8,            // u8の年齢フィールド
    }

    // implブロックを使うと対象の型(今回はStudent)に関連関数メソッドを実装できる
    impl Student {
        // 関連関数newを定義する(Student::new(...)の形式で呼び出せる)
        fn new(first_name: &str, last_name: &str, age: u8) -> Self {
            // 構造体を初期化して返す
            // Selfはimplの対象となっている型(Student)の別名
            Self {
                first_name: first_name.to_string(), // first_nameフィールドに値を設定
                last_name: last_name.to_string(),   // last_nameフィールドに値を設定
                age,                                // ageフィールドにage変数の値を設定
            }
        }
    }

    #[test]
    fn sort_u32_large() {
        {
            // 昇順
            // 2^16 = 65,536
            let mut x = new_u32_vec(65536);
            assert_eq!(sort(&mut x, &Ascending), Ok(()));

            // cargo test -- --nocaptureで出力できる
            /* for i in &x {
                print!("{}, ", i);
            } */
            // ソートが正しいことを確認する
            assert!(is_sorted_ascending(&x));
        }
        {
            // 降順
            let mut x = new_u32_vec(65536);
            assert_eq!(sort(&mut x, &Descending), Ok(()));
            assert!(is_sorted_descending(&x));
        }
    }

    #[test]
    fn sort_student_by_age_ascending() {
        // 4人分のテストデータ
        let taro = Student::new("Taro", "Yamada", 16);
        let hanako = Student::new("Hanako", "Yamada", 14);
        let kyoko = Student::new("Kyoko", "Ito", 15);
        let ryosuke = Student::new("Ryosuke", "Hayashi", 17);

        // ソート対象のベクタを作成
        let mut x = vec![&taro, &hanako, &kyoko, &ryosuke];

        // ソート後の期待値を作成
        let expected = vec![&hanako, &kyoko, &taro, &ryosuke];

        assert_eq!(sort_by(&mut x, &|a, b| a.age.cmp(&b.age)), Ok(()));
        assert_eq!(x, expected);
    }
    #[test]
    fn sort_student_by_name_ascending() {
        // 4人分のテストデータ
        let taro = Student::new("Taro", "Yamada", 16);
        let hanako = Student::new("Hanako", "Yamada", 14);
        let kyoko = Student::new("Kyoko", "Ito", 15);
        let ryosuke = Student::new("Ryosuke", "Hayashi", 17);

        // ソート対象のベクタを作成
        let mut x = vec![&taro, &hanako, &kyoko, &ryosuke];

        // ソート後の期待値を作成
        let expected = vec![&ryosuke, &kyoko, &hanako, &taro];

        assert_eq!(
            sort_by(&mut x, &|a, b| a
                .last_name
                .cmp(&b.last_name)
                .then_with(|| a.first_name.cmp(&b.first_name))),
            Ok(())
        );
        assert_eq!(x, expected);
    }

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
