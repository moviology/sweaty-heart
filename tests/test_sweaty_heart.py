from sweaty_heart import sum_as_string


def test_sum(benchmark):
    result = benchmark(sum_as_string, 2, 2)
    assert result == 4


def funk(a, b):
    return a + b


def test_sum2(benchmark):
    result = benchmark(funk, 2, 2)
    assert result == 4
