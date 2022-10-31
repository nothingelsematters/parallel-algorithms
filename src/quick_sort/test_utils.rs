pub(crate) fn correctness_test<F>(sort: F)
where
    F: Fn(&mut [u32]),
{
    for len in 0..1_000 {
        for _iter in 0..1_000 {
            let mut vec: Vec<u32> = (0..len).map(|_| rand::random::<u32>() % 1000).collect();
            let mut expected = vec.clone();
            expected.sort();

            sort(&mut vec);
            assert_eq!(expected, vec);
        }
    }
}
