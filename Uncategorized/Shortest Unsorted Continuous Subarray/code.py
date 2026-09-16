# import visualization libraries {
from algorithm_visualizer import Tracer, LogTracer, Array1DTracer, Layout, VerticalLayout
# }

# define tracer variables {
tracer = Array1DTracer('Sequence')
D = [2, 6, 4, 8, 10, 9, 15]
tracer.set(D)
logger = LogTracer()
Layout.setRoot(VerticalLayout([tracer, logger]))
Tracer.delay()
# }


def findUnsortedSubarray(nums):
    minValue = float('inf')
    maxValue = float('-inf')
    flag = False
    # visualize {
    minIndex = -1
    maxIndex = -1
    # }

    for i in range(1, len(nums)):
        # visualize {
        tracer.deselect(i - 2, i - 1)
        tracer.select(i - 1, i)
        Tracer.delay()
        # }

        if nums[i] < nums[i - 1]:
            flag = True
        if flag:
            minValue = min(minValue, nums[i])
            # visualize {
            if minValue == nums[i]:
                tracer.depatch(minIndex)
                minIndex = i
                tracer.patch(i)
            Tracer.delay()
            # }

    # visualize {
    tracer.depatch(minIndex)
    tracer.deselect(len(nums) - 2)
    tracer.deselect(len(nums) - 1)
    # }

    # logger {
    logger.println("min = {}".format(minValue))
    Tracer.delay()
    # }

    flag = False
    for i in range(len(nums) - 2, -1, -1):
        # visualize {
        tracer.deselect(i + 1, i + 2)
        tracer.select(i, i + 1)
        Tracer.delay()
        # }

        if nums[i] > nums[i + 1]:
            flag = True
        if flag:
            maxValue = max(maxValue, nums[i])
            # visualize {
            if maxValue == nums[i]:
                tracer.depatch(maxIndex)
                maxIndex = i
                tracer.patch(i)
            Tracer.delay()
            # }

    # visualize {
    tracer.depatch(maxIndex)
    tracer.deselect(0)
    tracer.deselect(1)
    Tracer.delay()
    # }

    # logger {
    logger.println("max = {}".format(maxValue))
    # }

    l = 0
    while l < len(nums):
        # visualize {
        tracer.deselect(l - 1)
        tracer.select(l)
        Tracer.delay()
        # }

        if minValue < nums[l]:
            # visualize {
            tracer.patch(l)
            Tracer.delay()
            # }
            break
        l += 1

    r = len(nums) - 1
    while r >= 0:
        # visualize {
        tracer.deselect(r + 1)
        tracer.select(r)
        Tracer.delay()
        # }

        if maxValue > nums[r]:
            # visualize {
            tracer.patch(r)
            Tracer.delay()
            # }
            break
        r -= 1

    # visualize {
    tracer.depatch(l)
    tracer.depatch(r)
    tracer.select(l, r)
    Tracer.delay()
    # }

    result = 0 if r - l < 0 else r - l + 1

    # logger {
    logger.println("result = {}".format(result))
    Tracer.delay()
    # }

    return result


findUnsortedSubarray(D)
