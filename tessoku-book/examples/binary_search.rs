fn main() {
    let arr1 = vec![0, 1, 2, 3, 5, 8, 13, 21];

    // binary_search関数
    // 該当する値が存在しない場合は、配列のどのインデックスに挿入することになるのかを計算する
    assert_eq!(arr1.binary_search(&1), Ok(1));
    assert_eq!(arr1.binary_search(&21), Ok(7));
    assert_eq!(arr1.binary_search(&4), Err(4));
    assert_eq!(arr1.binary_search(&100), Err(arr1.len()));

    // binary_search_by関数
    // std::cmp::Orderingを返却する関数を引数として指定する
    let elem1 = 15;
    assert_eq!(arr1.binary_search_by(|elem| elem.cmp(&elem1)), Err(7));
}
