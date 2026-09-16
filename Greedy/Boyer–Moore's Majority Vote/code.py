# import visualization libraries {
from algorithm_visualizer import Array1DTracer, LogTracer, Tracer, Layout, VerticalLayout
# }

A = [1, 3, 3, 2, 1, 1, 1]
N = len(A)

# define tracer variables {
tracer = Array1DTracer('List of element')
logger = LogTracer('Console')
Layout.setRoot(VerticalLayout([tracer, logger]))
tracer.set(A)
Tracer.delay()
# }


def isMajorityElement(element):
    count = 0
    # logger {
    logger.println("Verify majority element {}".format(element))
    # }
    for i in range(N - 1, -1, -1):
        # visualize {
        tracer.patch(i, A[i])
        Tracer.delay()
        # }
        if A[i] == element:
            count += 1
        else:
            # visualize {
            tracer.depatch(i)
            # }
    # logger {
    logger.println("Count of our assumed majority element {}".format(count))
    # }
    if count > N // 2:
        # logger {
        logger.println('Our assumption was correct!')
        # }
        return True
    # logger {
    logger.println('Our assumption was incorrect!')
    # }
    return False


def findProbableElement():
    index = 0
    count = 1
    # visualize {
    tracer.select(index)
    Tracer.delay()
    # }
    # logger {
    logger.println("Beginning with assumed majority element : {} count : {}".format(A[index], count))
    logger.println('--------------------------------------------------------')
    # }
    for i in range(1, N):
        # visualize {
        tracer.patch(i, A[i])
        Tracer.delay()
        # }
        if A[index] == A[i]:
            count += 1
            # logger {
            logger.println("Same as assumed majority element! Count : {}".format(count))
            # }
        else:
            count -= 1
            # logger {
            logger.println("Not same as assumed majority element! Count : {}".format(count))
            # }

        if count == 0:
            # logger {
            logger.println('Wrong assumption in majority element')
            # }
            # visualize {
            tracer.deselect(index)
            tracer.depatch(i)
            # }
            index = i
            count = 1
            # visualize {
            tracer.select(i)
            Tracer.delay()
            # }
            # logger {
            logger.println("New assumed majority element!{} Count : {}".format(A[i], count))
            logger.println('--------------------------------------------------------')
            # }
        else:
            # visualize {
            tracer.depatch(i)
            # }
    # logger {
    logger.println("Finally assumed majority element {}".format(A[index]))
    logger.println('--------------------------------------------------------')
    # }
    return A[index]


def findMajorityElement():
    element = findProbableElement()
    # logger {
    if isMajorityElement(element) is True:
        logger.println("Majority element is {}".format(element))
    else:
        logger.println('No majority element')
    # }


findMajorityElement()
