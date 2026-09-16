# import visualization libraries {
from algorithm_visualizer import Array1DTracer, LogTracer, Tracer, Layout, VerticalLayout
# }

ARank = {
    'Flavio': ['Valentine', 'July', 'Summer', 'Violet'],
    'Stephen': ['Summer', 'July', 'Valentine', 'Violet'],
    'Albert': ['July', 'Violet', 'Valentine', 'Summer'],
    'Jack': ['July', 'Violet', 'Valentine', 'Summer'],
}

BRank = {
    'July': ['Jack', 'Stephen', 'Albert', 'Flavio'],
    'Valentine': ['Flavio', 'Jack', 'Stephen', 'Albert'],
    'Violet': ['Jack', 'Stephen', 'Flavio', 'Albert'],
    'Summer': ['Stephen', 'Flavio', 'Albert', 'Jack'],
}

# define tracer variables {
tracerA = Array1DTracer('A')
tracerB = Array1DTracer('B')

_aKeys = list(ARank.keys())
_bKeys = list(BRank.keys())
tracerA.set(_aKeys)
tracerB.set(_bKeys)

logTracer = LogTracer('Console')
Layout.setRoot(VerticalLayout([tracerA, tracerB, logTracer]))
Tracer.delay()
# }


def init(rank):
    o = {}
    for k in rank:
        o[k] = {
            'key': k,
            'stable': False,
            'rankKeys': list(rank[k]),
        }
    return o


def extractUnstable(Q):
    for k in Q:
        if Q[k]['stable'] is False:
            return Q[k]
    return None


A = init(ARank)
B = init(BRank)

while True:
    a = extractUnstable(A)
    if a is None:
        break
    # logger {
    logTracer.println("Selecting {}".format(a['key']))
    Tracer.delay()
    # }

    bKey = a['rankKeys'].pop(0)
    b = B[bKey]

    # logger {
    logTracer.println("--> Choicing {}".format(b['key']))
    Tracer.delay()
    # }

    if b['stable'] is False:
        # logger {
        logTracer.println("--> {} is not stable, stabilizing with {}".format(b['key'], a['key']))
        Tracer.delay()
        # }

        a['stable'] = b
        b['stable'] = a

        # visualize {
        tracerA.select(_aKeys.index(a['key']))
        Tracer.delay()
        tracerB.select(_bKeys.index(b['key']))
        Tracer.delay()
        # }
    else:
        rankAinB = b['rankKeys'].index(a['key'])
        rankPrevAinB = b['rankKeys'].index(b['stable']['key'])
        if rankAinB < rankPrevAinB:
            # logger {
            logTracer.println("--> {} is more stable with {} rather than {} - stabilizing again".format(bKey, a['key'], b['stable']['key']))
            Tracer.delay()
            # }

            A[b['stable']['key']]['stable'] = False
            # visualize {
            tracerA.deselect(_aKeys.index(b['stable']['key']))
            Tracer.delay()
            # }

            a['stable'] = b
            b['stable'] = a

            # visualize {
            tracerA.select(_aKeys.index(a['key']))
            Tracer.delay()
            tracerB.select(_bKeys.index(b['key']))
            Tracer.delay()
            # }
