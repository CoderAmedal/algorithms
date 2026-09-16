# import visualization libraries {
from algorithm_visualizer import Array1DTracer, Array2DTracer, ChartTracer, Randomize, Tracer, Layout, VerticalLayout
# }

# define tracer variables {
chartTracer = ChartTracer('Chart')
arrayTracer = Array1DTracer('Array')
bucketsTracer = Array2DTracer('Buckets')
Layout.setRoot(VerticalLayout([chartTracer, arrayTracer, bucketsTracer]))
# }

# define input variables
N = 25  # the size of an array
K = 5  # the number of buckets
array = Randomize.Array1D(N=N, randomizer=Randomize.Integer(min=0, max=999)).create()

# create K buckets
buckets = [[] for _ in range(K)]
# visualize {
arrayTracer.chart(chartTracer)
arrayTracer.set(array)
bucketsTracer.set(buckets)
Tracer.delay()
# }

# find the maximum value that will be used for distribution
maximum = max(array)

# distribute the elements into the buckets
for i in range(N):
    number = array[i]
    bucketIndex = int(number / (maximum + 1) * K)
    bucket = buckets[bucketIndex]
    bucket.append(number)
    # visualize {
    arrayTracer.select(i)
    bucketsTracer.patch(bucketIndex, len(bucket) - 1, number)
    Tracer.delay()
    bucketsTracer.depatch(bucketIndex, len(bucket) - 1)
    # }

    # insertion sort within the bucket
    j = len(bucket) - 1
    while j > 0 and bucket[j - 1] > bucket[j]:
        bucket[j - 1], bucket[j] = bucket[j], bucket[j - 1]
        # visualize {
        bucketsTracer.patch(bucketIndex, j - 1, bucket[j - 1])
        bucketsTracer.patch(bucketIndex, j, bucket[j])
        Tracer.delay()
        bucketsTracer.depatch(bucketIndex, j - 1)
        bucketsTracer.depatch(bucketIndex, j)
        # }
        j -= 1
    # visualize {
    arrayTracer.deselect(i)
    # }

# concatenate the buckets back into the array
i = 0
for bucketIndex in range(K):
    bucket = buckets[bucketIndex]
    for j in range(len(bucket)):
        array[i] = bucket[j]
        # visualize {
        arrayTracer.patch(i, array[i])
        bucketsTracer.select(bucketIndex, j)
        Tracer.delay()
        bucketsTracer.deselect(bucketIndex, j)
        arrayTracer.depatch(i)
        # }
        i += 1
