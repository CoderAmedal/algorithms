# import visualization libraries {
from algorithm_visualizer import Array1DTracer, LogTracer, Tracer, Layout, VerticalLayout
# }

text = ['h', 'e', 'l', 'l', 'o', ' ', 's', 'i', 'r', ' ', 'h', 'e', 'l', 'l', 'o']
pattern = ['h', 'e', 'l', 'l', 'o']

Q = 101  # A prime number
D = 256  # number of characters in the input alphabet

# define tracer variables {
logger = LogTracer()
tracer1 = Array1DTracer('Text')
tracer2 = Array1DTracer('Pattern')
Layout.setRoot(VerticalLayout([logger, tracer1, tracer2]))
tracer1.set(text)
tracer2.set(pattern)
Tracer.delay()
# }

N = len(text)
M = len(pattern)

hashText = 0  # hash value for text
hashPattern = 0  # hash value for pattern
h = 1

for i in range(M - 1):
    h = (h * D) % Q

for i in range(M):
    hashPattern = (D * hashPattern + ord(pattern[i])) % Q
    hashText = (D * hashText + ord(text[i])) % Q

for i in range(N - M + 1):
    # Check if hash values of current window of text matches
    # with hash values of pattern. If match is found then
    # check for characters one by one
    if hashPattern == hashText:
        f = 0
        # visualize {
        tracer1.select(i, i + M - 1)
        Tracer.delay()
        tracer2.select(0, M - 1)
        Tracer.delay()
        # }
        for j in range(M):
            # visualize {
            tracer1.patch(i + j)
            Tracer.delay()
            tracer2.patch(j)
            Tracer.delay()
            # }
            if text[i + j] != pattern[j]:
                f += 1
            # visualize {
            tracer1.depatch(i + j)
            tracer2.depatch(j)
            # }

        # visualize {
        if f == 0:
            logger.println(" Pattern found at index {}".format(i))
        tracer1.deselect(i, i + M)
        tracer2.deselect(0, M - 1)
        # }

    # Calculate hash value for next window of text:
    if i < N - M:
        hashText = (D * (hashText - ord(text[i]) * h) + ord(text[i + M])) % Q

        # Convert negative value of hashText (if found) to positive
        if hashText < 0:
            hashText += Q
