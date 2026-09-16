# import visualization libraries {
from algorithm_visualizer import Array1DTracer, Tracer, Layout, VerticalLayout
# }

jobId = ['a', 'b', 'c', 'd', 'e']
deadline = [2, 1, 2, 1, 3]
profit = [100, 19, 27, 25, 15]
N = len(deadline)
# sort according to decreasing order of profit
# Bubble sort implemented ... Implement a better algorithm for better performance
for i in range(N - 1):
    for j in range(N - i - 1):
        if profit[j] < profit[j + 1]:
            profit[j], profit[j + 1] = profit[j + 1], profit[j]
            deadline[j], deadline[j + 1] = deadline[j + 1], deadline[j]
            jobId[j], jobId[j + 1] = jobId[j + 1], jobId[j]

slot = [0] * N
result = ['-'] * N

# define tracer variables {
tracer3 = Array1DTracer('Schedule')
tracer = Array1DTracer('Job Ids')
tracer1 = Array1DTracer('Deadlines')
tracer2 = Array1DTracer('Profit')
Layout.setRoot(VerticalLayout([tracer3, tracer, tracer1, tracer2]))
tracer.set(jobId)
tracer1.set(deadline)
tracer2.set(profit)
tracer3.set(result)
Tracer.delay()
# }

# Initialise all slots to free
for i in range(N):
    slot[i] = 0

# Iterate through all the given jobs
for i in range(N):
    # Start from the last possible slot.
    # Find a slot for the job
    # visualize {
    tracer.select(i)
    Tracer.delay()
    tracer1.select(i)
    Tracer.delay()
    # }
    for j in range(min(N, deadline[i]) - 1, -1, -1):
        if slot[j] == 0:
            # visualize {
            tracer3.patch(j, jobId[i])
            Tracer.delay()
            # }
            result[j] = jobId[i]
            slot[j] = 1
            # visualize {
            tracer3.depatch(j)
            # }
            break
    # visualize {
    tracer.deselect(i)
    tracer1.deselect(i)
    # }
