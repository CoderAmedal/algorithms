# import visualization libraries {
from algorithm_visualizer import Array1DTracer, Array2DTracer, Tracer, Layout, VerticalLayout
# }
string = "AAAABAABAAAABAAABAAAA"
pattern = "AAAABAAA"

_next = [0] * len(pattern)
# define tracer variables {
pattern_tracer = Array2DTracer('Pattern')
string_tracer = Array1DTracer('String')
Layout.setRoot(VerticalLayout([pattern_tracer, string_tracer]))
pattern_tracer.set([_next, pattern, pattern])
string_tracer.set(string)
Tracer.delay()
# }


def get_next(pattern):
    q = 1  # postfix pointer
    k = 0  # prefix pointer
    # visualize {
    pattern_tracer.select(2, k)
    # }
    while q < len(pattern):
        # visualize {
        pattern_tracer.select(1, q)
        Tracer.delay()
        # }
        while (k > 0) and (pattern[q] != pattern[k]):
            # visualize {
            pattern_tracer.select(0, k - 1)
            Tracer.delay()
            pattern_tracer.deselect(2, k)
            pattern_tracer.select(2, _next[k - 1])
            Tracer.delay()
            pattern_tracer.deselect(0, k - 1)
            # }
            k = _next[k - 1]
        if pattern[q] == pattern[k]:
            # visualize {
            pattern_tracer.deselect(2, k)
            pattern_tracer.select(2, k + 1)
            Tracer.delay()
            # }
            k += 1
        # visualize {
        pattern_tracer.patch(0, q, k)
        Tracer.delay()
        pattern_tracer.depatch(0, q)
        Tracer.delay()
        pattern_tracer.deselect(1, q)
        # }
        _next[q] = k
        q += 1
    # visualize {
    pattern_tracer.deselect(2, k)
    pattern_tracer.set([_next, pattern])
    Tracer.delay()
    # }


def kmp(string, pattern):
    match_positions = []

    i = 0  # string pointer
    k = 0  # pattern pointer
    get_next(pattern)
    while i < len(string):
        # visualize {
        string_tracer.select(i)
        pattern_tracer.select(1, k)
        Tracer.delay()
        # }
        while (k > 0) and (string[i] != pattern[k]):
            # visualize {
            pattern_tracer.select(0, k - 1)
            Tracer.delay()
            pattern_tracer.deselect(1, k)
            pattern_tracer.select(1, _next[k - 1])
            Tracer.delay()
            pattern_tracer.deselect(0, k - 1)
            # }
            k = _next[k - 1]
        if string[i] == pattern[k]:
            k += 1
            if k == len(pattern):
                match_start_position = i - len(pattern) + 1
                match_positions.append(match_start_position)
                # visualize {
                string_tracer.select(match_start_position, match_start_position + len(pattern) - 1)
                Tracer.delay()
                string_tracer.deselect(match_start_position, match_start_position + len(pattern) - 1)
                Tracer.delay()
                pattern_tracer.select(0, k - 1)
                Tracer.delay()
                pattern_tracer.deselect(1, k - 1)
                pattern_tracer.select(1, _next[k - 1])
                Tracer.delay()
                pattern_tracer.deselect(0, k - 1)
                # }
                k = _next[k - 1]
            else:
                # visualize {
                pattern_tracer.deselect(1, k - 1)
                pattern_tracer.select(1, k)
                Tracer.delay()
                # }
        else:
            # visualize {
            pattern_tracer.select(0, k)
            Tracer.delay()
            # }
        # visualize {
        pattern_tracer.deselect(0, k)
        pattern_tracer.deselect(1, k)
        string_tracer.deselect(i)
        # }
        i += 1
    # visualize {
    for j in range(len(match_positions)):
        string_tracer.select(match_positions[j], match_positions[j] + len(pattern) - 1)
        Tracer.delay()
        string_tracer.deselect(match_positions[j], match_positions[j] + len(pattern) - 1)
    # }


kmp(string, pattern)
