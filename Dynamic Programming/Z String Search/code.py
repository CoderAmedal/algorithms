# import visualization libraries {
from algorithm_visualizer import Array1DTracer, LogTracer, Tracer, Layout, VerticalLayout
# }

# let pattern = "aab";
# let text = "aabxaabxcaabxaabxay";
pattern = 'abc'
text = 'xabcabzabc'

length = len(pattern) + len(text) + 1

z = [0] * length

# define tracer variables {
textTracer = Array1DTracer('text')
pattTracer = Array1DTracer('pattern')
concatTracer = Array1DTracer('concatenated string')
tracer = Array1DTracer('zArray')
pattTracer.set(pattern)
textTracer.set(text)
tracer.set(z)
logger = LogTracer()
Layout.setRoot(VerticalLayout([textTracer, pattTracer, concatTracer, tracer, logger]))
Tracer.delay()
# }


def create_zarr(concat):
    left = 0
    right = 0
    N = len(concat)
    for i in range(1, N):
        # visualize {
        tracer.select(i)
        Tracer.delay()
        # }
        if i > right:
            left = right = i
            while right < N and concat[right] == concat[right - left]:
                # visualize {
                concatTracer.patch(right)
                concatTracer.select(right - left)
                logger.println("{} (at index {}) is equal to {} (at index {})".format(concat[right], right, concat[right - left], right - left))
                Tracer.delay()
                concatTracer.depatch(right)
                concatTracer.deselect(right - left)
                # }
                right += 1
            # visualize {
            if right < N:
                concatTracer.patch(right)
                concatTracer.select(right - left)
                logger.println("{} (at index {}) is NOT equal to {} (at index {})".format(concat[right], right, concat[right - left], right - left))
                Tracer.delay()
                concatTracer.depatch(right)
                concatTracer.deselect(right - left)
            # }
            z[i] = right - left
            # logger {
            logger.println('--------------------------------')
            logger.println("Value of z[{}] = the length of the substring starting from {} which is also the prefix of the concatinated string(={})".format(i, i, right - left))
            logger.println('--------------------------------')
            # }
            right -= 1
        elif z[i - left] < (right - i + 1):
            # visualize {
            logger.println("The substring from index {} will not cross the right end.".format(i - left))
            concatTracer.patch(right - i + 1)
            concatTracer.select(i - left)
            Tracer.delay()
            # }
            z[i] = z[i - left]
            # visualize {
            concatTracer.depatch(right - i + 1)
            concatTracer.deselect(i - left)
            # }
        else:
            # logger {
            logger.println("The substring from index {} will cross the right end.".format(i - left))
            # }
            left = i
            while right < N and concat[right] == concat[right - left]:
                # visualize {
                concatTracer.patch(right)
                concatTracer.select(right - left)
                logger.println("{} (at index {}) is equal to {} (at index {})".format(concat[right], right, concat[right - left], right - left))
                Tracer.delay()
                concatTracer.depatch(right)
                concatTracer.deselect(right - left)
                # }
                right += 1
            # visualize {
            if right < N:
                concatTracer.patch(right)
                concatTracer.select(right - left)
                logger.println("{} (at index {}) is NOT equal to {} (at index {})".format(concat[right], right, concat[right - left], right - left))
                Tracer.delay()
                concatTracer.depatch(right)
                concatTracer.deselect(right - left)
            # }
            z[i] = right - left
            right -= 1
            # logger {
            logger.println('--------------------------------')
            logger.println("Value of z[{}] = the length of the substring starting from {} which is also the prefix of the concatinated string(={})".format(i, i, right - left))
            logger.println('--------------------------------')
            # }
        # visualize {
        tracer.deselect(i)
        tracer.set(z)
        # }


concat = "{}${}".format(pattern, text)
# visualize {
concatTracer.set(concat)
# }
patLen = len(pattern)
create_zarr(concat)
# visualize {
tracer.set(z)
# }
# logger {
logger.println('The Values in Z array equal to the length of the pattern indicates the index at which the pattern is present')
logger.println('===================================')
for i in range(length):
    if z[i] == patLen:
        pos = i - (patLen + 1)
        logger.println("Pattern Found at index {}".format(pos))
logger.println('===================================')
# }
