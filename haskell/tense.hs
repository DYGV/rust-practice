-- tenseという関数は引数に整数値取り、文字列を返す。Haskellは型推論があるのでこれは書かなくても動く。
tense :: Integer -> String

{-
  以下のtenseは24時制から12時制へ変換する関数である。
  パイプ区切りでガード条件(条件式)を書き、その条件における式を書く。
  tenseという名前の関数でhourという名前の引数を取る。
  まず、where節でnotation_24とnotation_12にそれぞれhourを24、12で割った剰余をそれぞれ束縛する。以下のガード条件でこれらを使う。
  1つ目: notation_24が12以上の場合で、「n時は午後m時」という文字列にする。
  2つ目: notation_24が12より小さいの場合で、「n時は午前m時」という文字列にする。
　3つ目: 上記以外の時で、show関数でhourを文字列にする。今回は起こりえないものとしてエラーにする。 
-}
tense hour
    | notation_24 >= 12 = show hour ++ "時は午後"++ show notation_12 ++ "時"
    | notation_24 < 12  = show hour ++ "時は午前"++ show notation_12 ++ "時"
    | otherwise = error $ show hour
    where
      notation_24 = mod hour 24
      notation_12 = mod hour 12

{- 
  以下のmain関数内の$は「行末まで括弧(で囲む」、という意味でmain = putStr(unlines(map tense (take 35 [0..])))と書くのと同じである。
  
  今回、無限のリストを用いる必要は特にないが遅延評価はHaskellの特徴でもあるので使っていく。
  [0 ..]は0から開始する無限長のリストである(ただし遅延評価により無限のリストが作られるわけではない)。
  初めに、take関数により[0 ..]の先頭要素から35個の要素のリスト、つまり[0, 1,..., 34]を得る(これが評価)。
  次に、0から34までの要素を持つリストをmap関数ですべての要素に対しtenseを適用して新たなリストを作成する。
  その後、unlines関数で新しいリストの要素を改行文字を付加し、さらに結合する。
  最後に、putStr関数でその結合した文字列を出力する。
-}
main = putStr $ unlines $ map tense $ take 35 [0 ..]
