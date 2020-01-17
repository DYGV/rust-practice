use rand::distributions::Standard;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;

pub fn new_u32_vec(n: usize) -> Vec<u32> {
    // RNGを初期化する。再現性を持たせるために毎回同じシード値を使う
    let mut rng = Pcg64Mcg::from_seed([0; 16]);
    // n個の要素が格納できるようにベクタを初期化する
    // let mut v = Vec::with_capacity(n);

    // 0からn-1までの合計n回、繰り返し乱数を生成し、ベクタに格納する
    // 0からn-1の数列は使わないので。_で受け取ることですぐに破棄している
    /* for _ in 0..n {
        v.push(rng.sample(&Standard));
    }
    v */
    rng.sample_iter(&Standard).take(n).collect()
}

// 昇順にソートできていることを確認する関数
pub fn is_sorted_ascending<T: Ord>(x: &[T]) -> bool {
    // windows(2)は元のイテレータから1要素刻みで2要素ずつ値を取り出す
    // 新しいイテレータを返す
    // 例　 [1, 2, 3, 4] -> [1, 2], [2, 3], [3, 4]を順に返す
    // all(..)はイテレータから値(例: [1, 2])を取り出し、クロージャに渡す
    // クロージャがfalseを返したら、そこで処理を打ち切りfalseを返す
    // クロージャがtrueを返してる間は、イテレータから次の値を取り出し
    // クロージャへ与え続ける
    // イテレータの値が尽きるまでクロージャが一度もfalseを返さなかったら
    // all(..)はtrueを返す
    x.windows(2).all(|pair| pair[0] <= pair[1])
}

// 降順にソートできていることを確認する関数
pub fn is_sorted_descending<T: Ord>(x: &[T]) -> bool {
    x.windows(2).all(|pair| pair[0] >= pair[1])
}
