# import visualization libraries {
from algorithm_visualizer import Tracer, Array1DTracer, Array2DTracer, LogTracer, Layout, VerticalLayout
from functools import cmp_to_key
# }

word = 'virgo'
suffixArray = []
for i in range(1, len(word) + 2):
    suffixArray.append([i, '-'])

# define tracer variables {
saTracer = Array2DTracer('Suffix Array')
wordTracer = Array1DTracer('Given Word')
logger = LogTracer('Progress')
Layout.setRoot(VerticalLayout([saTracer, wordTracer, logger]))

saTracer.set(suffixArray)
wordTracer.set(list(word))
Tracer.delay()
# }

word += '$'  # special character
# logger {
logger.println("Appended '$' at the end of word as terminating (special) character. Beginning filling of suffixes")
# }


def selectSuffix(word, i):
    c = i

    while i < len(word) - 1:
        # visualize {
        wordTracer.select(i)
        # }
        i += 1
    # visualize {
    Tracer.delay()
    # }

    while c < len(word) - 1:
        # visualize {
        wordTracer.deselect(c)
        # }
        c += 1
    # visualize {
    Tracer.delay()
    # }


def createSA(sa, word):
    for i in range(len(word)):
        sa[i][1] = word[i:]

        selectSuffix(word, i)
        # visualize {
        saTracer.patch(i, 1, sa[i][1])
        Tracer.delay()
        saTracer.depatch(i, 1)
        Tracer.delay()
        # }


createSA(suffixArray, word)

# logger {
logger.println('Re-organizing Suffix Array in sorted order of suffixes using efficient sorting algorithm (O(N.log(N)))')
# }


def compare(a, b):
    # logger {
    logger.println("The condition a [1] ({}) > b [1] ({}) is {}".format(a[1], b[1], str(a[1] > b[1]).lower()))
    # }
    if a[1] > b[1]:
        return 1
    elif a[1] < b[1]:
        return -1
    return 0


suffixArray.sort(key=cmp_to_key(compare))

# visualize {
for i in range(len(word)):
    saTracer.patch(i, 0, suffixArray[i][0])
    saTracer.patch(i, 1, suffixArray[i][1])
    Tracer.delay()

    saTracer.depatch(i, 0)
    saTracer.depatch(i, 1)
# }
