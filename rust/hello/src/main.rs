fn main() {
  println!("Hello, world!");
  // {:.1} で小数第一位まで表示
  println!("{:.1}", add(12.345, 12.345)); // add関数の戻り値を表示
  println!("Hello {}", "DYGV",); // フォーマット文字列
                                 // フォーマット文字列と数値
  println!(
    "半径: {:.1}、円周率: {:.3}、面積{:.3}",
    3.2,
    std::f64::consts::PI,
    3.2f64.powi(2) * std::f64::consts::PI,
  );

  // RPN(逆ポーランド記法)計算機プログラムとデバッガによる実行
  let exp = "6.1 5.2 4.3 * + 3.4 2.5 / 1.6 * -";
  let ans = rpn(exp);
  debug_assert_eq!("26.2840", format!("{:.4}", ans));
  println!("{} = {:.4}", exp, ans);
}

// fn 関数名(引数1: 型1, 引数2: 型2, ...)-> 戻り値の型 {}
fn add(x: f64, y: f64) -> f64 {
  // x, yをそれぞれf64で受け取りf64で返す
  x + y // 最後に書いた式の評価結果が関数の戻り値となる
}

// RPN形式の文字列expを受け取りf64型の計算結果を返す
fn rpn(exp: &str) -> f64 {
  // 変数stackを空のスタックに束縛
  // mut = mutable
  let mut stack = Vec::new();
  // expの要素をスペースで分割し、tokenをそれらに順に束縛
  // 要素がなくなるまで繰り返す
  for token in exp.split_whitespace() {
    // tokenがf64型の数値ならスタックに積む
    if let Ok(num) = token.parse::<f64>() {
      stack.push(num);
    } else {
      match token {
        "+" => apply2(&mut stack, |x, y| x + y),
        "-" => apply2(&mut stack, |x, y| x - y),
        "*" => apply2(&mut stack, |x, y| x * y),
        "/" => apply2(&mut stack, |x, y| x / y),
        // トークンが演算子でないときはエラーを起こして終了
        _ => panic!("Unknown operator: {}", token),
      }
    }
  }
  // スタックから1個数値を取り出し、失敗したらエラーを起こして終了
  stack.pop().expect("Stack Underflow")
}

// スタックから2つ数値を取り出し、F型のクロージャfunで計算し、結果をスタックに積む
fn apply2<F>(stack: &mut Vec<f64>, fun: F)
// F型のトレイと境界
where
  F: Fn(f64, f64) -> f64,
{
  // 2つ数値を取り出す(変数y, xをスタックの最後に束縛)
  if let (Some(y), Some(x)) = (stack.pop(), stack.pop()) {
    let z = fun(x, y);
    // 変数zの値をstackに積む
    stack.push(z);
  } else {
    // スタックから要素が取り出せなかったときはエラーを起こして終了
    panic!("Stack underflow");
  }
}
