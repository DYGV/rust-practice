"Bitonic merge sort"


def sort(x, isAscending):
    """
    xの要素数は2のべき乗である必要がある
    要素数が1以下になったらおわり
    """
    if len(x) <= 1:
        return x
    else:
        # リストの前半(first)は昇順、後半(second)は降順でソートする
        mid_point = len(x) // 2
        # first, secondを再帰的にsort()に渡す
        first = sort(x[:mid_point], True)
        second = sort(x[mid_point:], False)

        # 結合し、サブソートに進む
        x1 = first + second
        return _sub_sort(x1, isAscending)


def _sub_sort(x, isAscending):
    """
    バイトニック列xの前半と後半を指定された向きに比較、交換し
    前半と後半それぞれについて再帰的にサブソートをする
    """
    if len(x) <= 1:
        return x
    else:
        _compare_and_swap(x, isAscending)

        mid_point = len(x) // 2
        first = _sub_sort(x[:mid_point], isAscending)
        second = _sub_sort(x[mid_point:], isAscending)

        return first + second


def _compare_and_swap(x, isAscending):
    """
    バイトニック列を要素数/2要素ごとに比較して
    指定された順序になるように交換する
    """
    mid_point = len(x) // 2
    for i in range(mid_point):
        if (x[i] > x[mid_point + i]) == isAscending:
            x[i], x[mid_point + i] = x[mid_point + i], x[i]


if __name__ == "__main__":
    num = [10, 30, 11, 20, 4, 330, 21, 110]  # ソート対象の数列
    isAscending = True
    sort = sort(num, isAscending)
    print(sort)
