# import visualization libraries {
from algorithm_visualizer import Array2DTracer, LogTracer, Tracer, Layout, VerticalLayout
# }

str1 = 'stack'
str2 = 'racket'
table = [[-1] * (len(str2) + 1) for _ in range(len(str1) + 1)]

for i in range(len(str1) + 1):
    table[i][0] = i
for i in range(1, len(str2) + 1):
    table[0][i] = i

# define tracer variables {
tracer = Array2DTracer('Distance Table')
logger = LogTracer()
Layout.setRoot(VerticalLayout([tracer, logger]))
tracer.set(table)
Tracer.delay()
# }

# logger {
logger.println('Initialized DP Table')
logger.println("Y-Axis (Top to Bottom): {}".format(str1))
logger.println("X-Axis (Left to Right): {}".format(str2))
# }


def edit_distance(str1, str2, table):
    # display grid with words
    # logger {
    logger.println("*** {}".format(" ".join(str2)))
    for index, item in enumerate(table):
        character = '*' if index == 0 else str1[index - 1]
        logger.println("{}\t[{}]".format(character, ", ".join(map(str, item))))
    # }

    # begin ED execution
    for i in range(1, len(str1) + 1):
        for j in range(1, len(str2) + 1):
            if str1[i - 1] == str2[j - 1]:
                # visualize {
                tracer.select(i - 1, j - 1)
                Tracer.delay()
                # }
                table[i][j] = table[i - 1][j - 1]
                # visualize {
                tracer.patch(i, j, table[i][j])
                Tracer.delay()
                tracer.depatch(i, j)
                tracer.deselect(i - 1, j - 1)
                # }
            else:
                # visualize {
                tracer.select(i - 1, j)
                tracer.select(i, j - 1)
                tracer.select(i - 1, j - 1)
                Tracer.delay()
                # }
                table[i][j] = min(table[i - 1][j], table[i][j - 1], table[i - 1][j - 1]) + 1
                # visualize {
                tracer.patch(i, j, table[i][j])
                Tracer.delay()
                tracer.depatch(i, j)
                tracer.deselect(i - 1, j)
                tracer.deselect(i, j - 1)
                tracer.deselect(i - 1, j - 1)
                # }
    # visualize {
    tracer.select(len(str1), len(str2))
    # }
    return table[len(str1)][len(str2)]


dist = edit_distance(str1, str2, table)

# logger {
logger.println("Minimum Edit Distance: {}".format(dist))
# }
