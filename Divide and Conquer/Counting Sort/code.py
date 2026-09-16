# import visualization libraries {
from algorithm_visualizer import Array1DTracer, Randomize, Tracer, Layout, VerticalLayout
# }

# define tracer variables {
arrayTracer = Array1DTracer('Array')
countsTracer = Array1DTracer('Counts')
sortedArrayTracer = Array1DTracer('Sorted Array')
Layout.setRoot(VerticalLayout([arrayTracer, countsTracer, sortedArrayTracer]))
# }

# define input variables
N = 20  # the size of an array
array = Randomize.Array1D(N=N, randomizer=Randomize.Integer(min=0, max=9)).create()

# find the maximum value that will decide the size of counts array
maximum = max(array)
counts = [0] * (maximum + 1)
# visualize {
arrayTracer.set(array)
countsTracer.set(counts)
Tracer.delay()
# }

# store counts of each number
for i in range(N):
    number = array[i]
    counts[number] += 1
    # visualize {
    arrayTracer.select(i)
    countsTracer.patch(number, counts[number])
    Tracer.delay()
    countsTracer.depatch(number)
    arrayTracer.deselect(i)
    # }

# calculate the prefix sums
for i in range(1, maximum + 1):
    counts[i] += counts[i - 1]
    # visualize {
    countsTracer.select(i - 1)
    countsTracer.patch(i, counts[i])
    Tracer.delay()
    countsTracer.depatch(i)
    countsTracer.deselect(i - 1)
    # }

# create a sorted array based on the prefix sums
sortedArray = [0] * N
# visualize {
sortedArrayTracer.set(sortedArray)
# }
for i in range(N - 1, -1, -1):
    number = array[i]
    count = counts[number]
    sortedArray[count - 1] = number
    counts[number] -= 1
    # visualize {
    arrayTracer.select(i)
    countsTracer.select(number)
    sortedArrayTracer.patch(count - 1, sortedArray[count - 1])
    countsTracer.patch(number, counts[number])
    Tracer.delay()
    sortedArrayTracer.depatch(count - 1)
    countsTracer.depatch(number)
    countsTracer.deselect(number)
    arrayTracer.deselect(i)
    # }
