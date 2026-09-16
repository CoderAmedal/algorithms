# import visualization libraries {
from algorithm_visualizer import Tracer, Array1DTracer, LogTracer, Layout, VerticalLayout
# }

plainText = 'secret'

# define tracer variables {
ptTracer = Array1DTracer('Encryption')
ctTracer = Array1DTracer('Decryption')
logger = LogTracer()
Layout.setRoot(VerticalLayout([ptTracer, ctTracer, logger]))

ptTracer.set(list(plainText))
Tracer.delay()
# }

# code assumes that plainText contains ONLY LOWER CASE ALPHABETS

keys = {'a': 5, 'b': 7}
N = 26


def encrypt(text):
    cypherText = ''

    def cryptAlpha(alpha):
        index = ord(alpha) - ord('a')
        result = ((keys['a'] * index) + keys['b']) % N

        # logger {
        logger.println("Index of {} = {}".format(alpha, index))
        # }

        result += ord('a')
        return chr(result)

    # logger {
    logger.println('Beginning Affine Encryption')
    logger.println('Encryption formula: <b>((keys.a * indexOfAlphabet) + keys.b) % N</b>')
    logger.println("keys.a={}, keys.b={}, N={}".format(keys['a'], keys['b'], N))
    # }

    for i in range(len(text)):
        # visualize {
        ptTracer.select(i)
        Tracer.delay()
        ptTracer.deselect(i)
        # }

        cypherText += cryptAlpha(text[i])

        # visualize {
        ptTracer.patch(i, cypherText[-1])
        Tracer.delay()
        ptTracer.depatch(i)
        # }

    return cypherText


def decrypt(cypherText):
    text = ''
    aInverse = 0
    for i in range(1, N):
        if (keys['a'] * i) % N == 1:
            aInverse = i
            break

    # logger {
    logger.println("a<sup>-1</sup> = {}".format(aInverse))
    # }

    def decryptAlpha(alpha):
        index = ord(alpha) - ord('a')
        result = (aInverse * (index - keys['b'])) % N

        # logger {
        logger.println("Index of {} = {}".format(alpha, index))
        # }

        result += ord('a')
        return chr(result)

    # logger {
    logger.println('Beginning Affine Decryption')
    logger.println('Decryption formula: <b>(a<sup>-1</sup> * (index - keys.b)) % N</b>')
    logger.println("keys.b={}, N={}".format(keys['b'], N))
    # }

    for i in range(len(cypherText)):
        # visualize {
        ctTracer.select(i)
        Tracer.delay()
        ctTracer.deselect(i)
        Tracer.delay()
        # }

        text += decryptAlpha(cypherText[i])

        # visualize {
        ctTracer.patch(i, text[-1])
        Tracer.delay()
        ctTracer.depatch(i)
        Tracer.delay()
        # }

    return text


cipherText = encrypt(plainText)
ctTracer.set(list(cipherText))
decrypt(cipherText)
