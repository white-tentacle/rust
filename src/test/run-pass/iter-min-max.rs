fn is_even(&&x: uint) -> bool { (x % 2u) == 0u }

fn main() {
    assert [1u, 3u]/_.min() == 1u;
    assert [3u, 1u]/_.min() == 1u;
    assert Some(1u).min() == 1u;

    assert [1u, 3u]/_.max() == 3u;
    assert [3u, 1u]/_.max() == 3u;
    assert Some(3u).max() == 3u;
}
