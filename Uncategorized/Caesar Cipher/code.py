# import visualization libraries {
from algorithm_visualizer import Tracer, Array1DTracer, LogTracer, Layout, VerticalLayout
# }

string = 'hello! how are you doing?'
rotation = 5
alphabet = 'abcdefghijklmnopqrstuvwxyz'
# create a map of char -> position to improve run time
alphabetMap = {ch: idx for idx, ch in enumerate(alphabet)}

# define tracer variables {
encryptTracer = Array1DTracer('Encryption')
decryptTracer = Array1DTracer('Decryption')
logger = LogTracer()
Layout.setRoot(VerticalLayout([encryptTracer, decryptTracer, logger]))

encryptTracer.set(list(string))
Tracer.delay()
# }


def getPosUp(pos):
    return 0 if pos == len(alphabet) - 1 else pos + 1


def getPosDown(pos):
    return len(alphabet) - 1 if pos == 0 else pos - 1


def getNextChar(currChar, direction):
    pos = alphabetMap[currChar]
    nextPos = getPosUp(pos) if direction == 'up' else getPosDown(pos)
    nextChar = alphabet[nextPos]

    # logger {
    logger.println("{} -> {}".format(currChar, nextChar))
    # }
    return nextChar


def cipher(text, rotation, direction, cipherTracer):
    if not text:
        return ''

    chars = list(text)
    for i in range(len(chars)):
        # visualize {
        Tracer.delay()
        # }

        currChar = chars[i]
        if currChar in alphabetMap:  # don't encrypt/decrypt characters not in alphabetMap
            r = rotation

            # logger {
            logger.println("Rotating {} {} {} times".format(currChar, direction, rotation))
            # }
            # visualize {
            cipherTracer.select(i)
            Tracer.delay()
            # }

            # perform given amount of rotations in the given direction
            while r > 0:
                r -= 1
                currChar = getNextChar(currChar, direction)
                # visualize {
                cipherTracer.patch(i, currChar)
                Tracer.delay()
                # }
        else:
            # logger {
            logger.println('Ignore this character')
            # }

        chars[i] = currChar
        # logger {
        logger.println("Current result: {}".format(''.join(chars)))
        # }

    return ''.join(chars)


def encrypt(text, rotation):
    # logger {
    logger.println("Encrypting: {}".format(text))
    # }
    return cipher(text, rotation, 'up', encryptTracer)


def decrypt(text, rotation):
    # logger {
    logger.println("Decrypting: {}".format(text))
    # }
    return cipher(text, rotation, 'down', decryptTracer)


encrypted = encrypt(string, rotation)
# logger {
logger.println("Encrypted result: {}".format(encrypted))
# }

decryptTracer.set(list(encrypted))
decrypted = decrypt(encrypted, rotation)
# logger {
logger.println("Decrypted result: {}".format(decrypted))
# }
