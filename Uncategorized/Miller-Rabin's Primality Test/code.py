# import visualization libraries {
from algorithm_visualizer import Tracer, LogTracer, Layout, VerticalLayout
import random
# }

# define tracer variables {
logger = LogTracer()
Layout.setRoot(VerticalLayout([logger]))
Tracer.delay()
# }


# Utility function to do modular exponentiation.
# It returns (x^y) % p
def power(x, y, p):
    res = 1
    x %= p
    while y > 0:
        # If y is odd, multiply x with result
        if y & 1:
            res = (res * x) % p
        # y must be even now
        y >>= 1  # y = y/2
        x = (x * x) % p
    return res


# Determine if N is prime using Miller-Rabin probabilistic algorithm
def testProbablyPrime(n, k=5):
    # visualize {
    logger.println("==> Testing number {}".format(n))
    # }

    if n == 1 or n == 3:
        # visualize {
        logger.println('==> Simple case, N is 1 or 3')
        # }
        return True
    if n % 2 == 0:
        # visualize {
        logger.println("==> Simple case, {} mod 2 = 0".format(n))
        # }
        return False

    # Write (n - 1) as 2^s * d
    d = n - 1
    while d % 2 == 0:
        d //= 2
    # visualize {
    logger.println("d = {}".format(d))
    # }

    P = 100 * (1 - (1 / (4 ** k)))

    while True:
        # visualize {
        logger.println("Remaining iterations: #{}".format(k))
        # }

        a = 2 + int(random.random() * (n - 4))
        # visualize {
        logger.println("--> first test with random = {}".format(a))
        # }

        # Compute a^d % n
        x = power(a, d, n)

        if x == 1 or x == n - 1:
            # visualize {
            logger.println('--> continue WitnessLoop, x = 1 or x = n-1')
            # }
            k -= 1
            if k == 0:
                break
            continue

        # visualize {
        logger.println('--> second test')
        # }

        # Keep squaring x while one of the following doesn't happen
        # (i)   d does not reach n-1
        # (ii)  (x^2) % n is not 1
        # (iii) (x^2) % n is not n-1
        i = d
        continued = False
        while i != n - 1:
            x = (x * x) % n
            i *= 2

            if x == 1:
                # visualize {
                logger.println("--> exiting, {} is composite".format(n))
                # }
                return False

            if x == n - 1:
                # visualize {
                logger.println('--> continue WitnessLoop')
                # }
                continued = True
                break

        if continued:
            k -= 1
            if k == 0:
                break
            continue

        # visualize {
        logger.println("--> exiting, {} is composite 'cause (n-1) is reached".format(n))
        # }
        return False

    # visualize {
    logger.println("End of tests, {} is probably prime with probabilty of {}%".format(n, P))
    # }
    return True


for i in range(3):
    a = int(random.random() * 300)
    if a % 2 == 0:
        a += 1
    testProbablyPrime(a)
    # visualize {
    logger.println('----------')
    # }

testProbablyPrime(151)
# visualize {
logger.println('----------')
# }

testProbablyPrime(199, 10)
