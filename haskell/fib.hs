-- 項数からフィボナッチ数を求めるプログラム

-- fib 0は0である(項0番目は0である)
fib 0 = 0
-- fib 1は1である(項1番目は1である)
fib 1 = 1
-- fib nはfib (n-2) + fib (n-1)である
fib n = fib (n-2) + fib (n-1)

-- 6項目を求め、それを文字列にして出力
main = putStr $ show $ fib 6
