# import visualization libraries {
from algorithm_visualizer import Array1DTracer, LogTracer, Randomize, Tracer, Layout, VerticalLayout
# }

# define tracer variables {
array1d_tracer = Array1DTracer("Set")
log_tracer = LogTracer("Console")
# }

# define input variables
n = 10
s = []
d = 0


def solve():
    sel = [0] * (n + 1)
    k = 0
    total = 0
    found = 0
    sel[0] = 1
    array1d_tracer.select(k)
    Tracer.delay()
    while True:
        if k < n and sel[k] == 1:
            if total + s[k] == d:
                found = 1
                log_tracer.print("{")
                for i in range(n):
                    if sel[i] == 1:
                        log_tracer.print("{}  ".format(s[i]))
                log_tracer.println("}")
                sel[k] = 0
                Tracer.delay()
                array1d_tracer.deselect(k)
                Tracer.delay()
            elif total + s[k] < d:
                total += s[k]
            else:
                sel[k] = 0
                array1d_tracer.deselect(k)
                Tracer.delay()
        else:
            k -= 1
            while k >= 0 and sel[k] == 0:
                k -= 1
            if k < 0:
                break
            sel[k] = 0
            array1d_tracer.deselect(k)
            Tracer.delay()
            total -= s[k]
        k += 1
        if k < n:
            sel[k] = 1
            array1d_tracer.select(k)
            Tracer.delay()
    if found == 0:
        log_tracer.println("Not possible subsets")


# visualize {
Layout.setRoot(VerticalLayout([array1d_tracer, log_tracer]))
Tracer.delay()
# }
n = 10
# Randomizing the array{
s = Randomize.Array1D(N=n, randomizer=Randomize.Integer(min=0, max=29)).create()
d = Randomize.Integer(min=0, max=99).create()
# }

log_tracer.print("The Given set is: ")
for x in s:
    log_tracer.print(str(x) + ",")
log_tracer.println("\nDesired sum is:" + str(d))
log_tracer.println("The possible subsets of sum " + str(d) + " are: ")
array1d_tracer.set(s)
Tracer.delay()
solve()
