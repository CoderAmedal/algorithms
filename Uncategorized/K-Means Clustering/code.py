# import visualization libraries {
from algorithm_visualizer import Array2DTracer, LogTracer, Randomize, Tracer, Layout, VerticalLayout
import random
# }

# define helper functions {
def shuffle(a):
    array = list(a)
    copy = []
    n = len(array)

    while n:
        i = random.randrange(n)
        n -= 1
        copy.append(array.pop(i))

    return copy


def add(x, y):
    return x + y


def chooseRandomCenters(data, k):
    return shuffle(data)[:k]


def pointify(p):
    return "({}, {})".format(p[0], p[1])


def arrayify(a):
    return [pointify(p) for p in a]


def stringify(a):
    return ", ".join(arrayify(a))


def distance(p1, p2):
    return add((p1[0] - p2[0]) ** 2, (p1[1] - p2[1]) ** 2)


def col(a, i):
    return [p[i] for p in a]


def mean(a):
    if not a:
        return 0
    return sum(a) / len(a)


def centerOfCluster(cluster):
    return [mean(col(cluster, 0)), mean(col(cluster, 1))]


def reCalculateCenters(clusters):
    return [centerOfCluster(c) for c in clusters]


def areCentersEqual(c1, c2):
    return c1 is not None and c2 is not None and c1 == c2


def cluster(data, centers):
    clusters = [[] for _ in centers]

    for i in range(len(data)):
        point = data[i]
        minDistance = float('inf')
        minDistanceIndex = -1

        for j in range(len(centers)):
            d = distance(point, centers[j])

            if d < minDistance:
                minDistance = d
                minDistanceIndex = j

        clusters[minDistanceIndex].append(point)

    return clusters
# }

# define tracer variables {
array2dTracer = Array2DTracer('Grid')
logTracer = LogTracer('Console')
# }

# define input variables
unClusteredData = [row[:2] for row in Randomize.Array2D(N=15, M=2).create()]
k = random.randint(2, max(2, len(unClusteredData) // 5))


def recenterAndCluster(originalClusters):
    centers = reCalculateCenters(originalClusters)
    clusters = cluster(unClusteredData, centers)
    return centers, clusters


def improve(loops, clusters, centers):
    def allowImprove():
        return loops < 1000

    while True:
        if not allowImprove():
            return clusters, centers

        loops += 1

        retCenters, retClusters = recenterAndCluster(clusters)

        # trace {
        array2dTracer.set([arrayify(c) for c in retClusters])

        logTracer.println('')
        logTracer.println("Iteration #{} Result: ".format(loops))
        logTracer.println('\tClusters:')
        logTracer.println("\t\t{}".format("\n\t\t".join(stringify(c) for c in retClusters)))
        logTracer.println('\tCenters:')
        logTracer.println("\t\t{}".format(stringify(retCenters)))
        logTracer.println('')

        Tracer.delay()
        # }

        if not allowImprove() or areCentersEqual(centers, retCenters):
            return retClusters, retCenters

        clusters = retClusters
        centers = retCenters


# visualize {
Layout.setRoot(VerticalLayout([array2dTracer, logTracer]))

logTracer.println("Un-clustered data = {}".format(stringify(unClusteredData)))
array2dTracer.set([arrayify(unClusteredData)])

Tracer.delay()
# }

# Start with random centers
centers = chooseRandomCenters(unClusteredData, k)

# trace {
logTracer.println("Initial random selected centers = {}".format(stringify(centers)))

Tracer.delay()
# }

# Cluster to the random centers
clusters = cluster(unClusteredData, centers)

# trace {
logTracer.println("Initial clusters = \n\t{}".format("\n\t".join(stringify(c) for c in clusters)))
array2dTracer.set([arrayify(c) for c in clusters])

Tracer.delay()
# }

# start iterations here
retClusters, retCenters = improve(0, clusters, centers)

# trace {
Tracer.delay()

logTracer.println("Final clustered data = \n\t{}".format("\n\t".join(stringify(c) for c in retClusters)))
logTracer.println("Best centers = {}".format(stringify(retCenters)))
array2dTracer.set([arrayify(c) for c in retClusters])
Tracer.delay()
# }
