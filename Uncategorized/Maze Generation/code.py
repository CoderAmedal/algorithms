# import visualization libraries {
from algorithm_visualizer import Tracer, Array2DTracer, LogTracer, Layout, VerticalLayout
import random
# }

n = 6  # rows (change these!)
m = 6  # columns (change these!)

hEnd = m * 4 - (m - 1)
vEnd = n * 3 - (n - 1)

G = []

for i in range(vEnd):  # by row
    G.append([' '] * hEnd)
    for j in range(hEnd):  # by column
        G[i][j] = ' '

        if i == 0 and j == 0:  # top-left corner
            G[i][j] = '\u250c'
        elif i == 0 and j == hEnd - 1:  # top-right corner
            G[i][j] = '\u2510'
        elif i == vEnd - 1 and j == 0:  # bottom-left corner
            G[i][j] = '\u2514'
        elif i == vEnd - 1 and j == hEnd - 1:  # bottom-right corner
            G[i][j] = '\u2518'
        elif (j % 3 == 0) and (i % vEnd != 0 and i != vEnd - 1 and i % 2 == 1):
            G[i][j] = '\u2502'
        elif i % 2 == 0:
            G[i][j] = '\u2500'

        if m > 1:  # More than one column
            if j % 3 == 0 and j != 0 and j != hEnd - 1 and i == 0:
                G[i][j] = '\u252c'
            if j % 3 == 0 and j != 0 and j != hEnd - 1 and i == vEnd - 1:
                G[i][j] = '\u2534'

        if n > 1:  # More than one row
            if i % 2 == 0 and i != 0 and i != vEnd - 1 and j == 0:
                G[i][j] = '\u251c'
            if i % 2 == 0 and i != 0 and i != vEnd - 1 and j == hEnd - 1:
                G[i][j] = '\u2524'

        if n > 1 and m > 1:  # More than one row and column
            if i % 2 == 0 and j % 3 == 0 and i != 0 and j != 0 and i != vEnd - 1 and j != hEnd - 1:
                G[i][j] = '\u253c'

# define tracer variables {
tracer = Array2DTracer()
logger = LogTracer()
Layout.setRoot(VerticalLayout([tracer, logger]))
tracer.set(G)
Tracer.delay()
# }


class disjointSet:
    def __init__(self):
        self.set = []
        self.elements = 0

    def addElements(self, numberOfElements):
        for _ in range(numberOfElements):
            self.elements += 1
            self.set.append(-1)

    def find(self, element):
        if self.set[element] < 0:
            return element
        self.set[element] = self.find(self.set[element])
        return self.set[element]

    def setUnion(self, _a, _b):
        a = self.find(_a)
        b = self.find(_b)

        if a != b:
            newSize = self.set[a] + self.set[b]
            if self.compareSize(a, b):
                self.set[b] = a
                self.set[a] = newSize
            else:
                self.set[a] = b
                self.set[b] = newSize

    def compareSize(self, a, b):
        if self.set[a] == self.set[b]:
            return True
        elif self.set[a] < self.set[b]:
            return True
        return False


# http://bost.ocks.org/mike/shuffle/
def shuffle(array):
    m = len(array)
    # While there remain elements to shuffle...
    while m:
        # Pick a remaining element...
        i = random.randrange(m)
        m -= 1
        # And swap it with the current element.
        array[m], array[i] = array[i], array[m]
    return array


def buildMaze():
    mySet = disjointSet()
    width = m
    height = n
    setSize = 0
    graph = [[0] * height for _ in range(width)]
    visitedMap = {}
    walls = {}
    rightWalls = []
    downWalls = []
    location = 0

    mySet.addElements(width * height)

    # logger {
    logger.println('initializing grid (all walls are up)')
    # }
    # init 'graph'
    # each room has two walls, a down and right wall.
    for i in range(width):
        for j in range(height):
            graph[i][j] = location

            walls[location] = {'down': True, 'right': True}
            visitedMap[location] = False

            # If you can label the rooms with just 2 digits
            if width * height < 100:
                locationString = str(location)

                G[j * 2 + 1][i * 3 + 1] = locationString[0]
                G[j * 2 + 1][i * 3 + 2] = locationString[1] if len(locationString) > 1 else ' '

                # visualize {
                tracer.set(G)
                # }

            rightWalls.append({'x': i, 'y': j})
            downWalls.append({'x': i, 'y': j})
            location += 1

    # logger {
    logger.println('shuffled the walls for random selection')
    # }
    # Randomly shuffle the walls
    shuffle(rightWalls)
    shuffle(downWalls)

    # Picking random walls to remove
    while setSize != mySet.elements - 1:
        randomWall = random.randint(1, 2)
        if randomWall == 1 and len(downWalls) > 0:
            # Down wall
            currentRoom = downWalls.pop()
            iX = currentRoom['x']
            iY = currentRoom['y']
            iYdown = iY + 1
            if iYdown < height:
                u = graph[iX][iY]
                v = graph[iX][iYdown]
                # visualize {
                tracer.patch(iY * 2 + 1, iX * 3 + 1)
                tracer.patch(iY * 2 + 1, iX * 3 + 2)
                tracer.patch(iYdown * 2 + 1, iX * 3 + 1)
                tracer.patch(iYdown * 2 + 1, iX * 3 + 2)
                # }
                if mySet.find(u) != mySet.find(v):
                    # logger {
                    logger.println("Rooms: {} & {} now belong to the same set, delete wall between them".format(u, v))

                    Tracer.delay()
                    # }
                    mySet.setUnion(u, v)
                    setSize += 1
                    # delete wall
                    walls[u]['down'] = False
                else:
                    # logger {
                    logger.println("Rooms: {} & {} would create a cycle! This is not good!".format(u, v))
                    Tracer.delay()
                    # }
                # visualize {
                tracer.depatch(iY * 2 + 1, iX * 3 + 1)
                tracer.depatch(iY * 2 + 1, iX * 3 + 2)
                tracer.depatch(iYdown * 2 + 1, iX * 3 + 1)
                tracer.depatch(iYdown * 2 + 1, iX * 3 + 2)
                # }
        elif randomWall == 2 and len(rightWalls) > 0:
            # Right Wall
            currentRoom = rightWalls.pop()
            iX = currentRoom['x']
            iY = currentRoom['y']
            iXright = iX + 1
            if iXright < width:
                u = graph[iX][iY]
                v = graph[iXright][iY]
                # visualize {
                tracer.patch(iY * 2 + 1, iX * 3 + 1)
                tracer.patch(iY * 2 + 1, iX * 3 + 2)
                tracer.patch(iY * 2 + 1, iXright * 3 + 1)
                tracer.patch(iY * 2 + 1, iXright * 3 + 2)
                # }
                if mySet.find(u) != mySet.find(v):
                    # logger {
                    logger.println("Rooms: {} & {} now belong to the same set, delete wall between them".format(u, v))

                    Tracer.delay()
                    # }
                    mySet.setUnion(u, v)
                    setSize += 1
                    # delete wall
                    walls[u]['right'] = False
                else:
                    # logger {
                    logger.println("Rooms: {} & {} would create a cycle! This is not good!".format(u, v))
                    Tracer.delay()
                    # }
                # visualize {
                tracer.depatch(iY * 2 + 1, iX * 3 + 1)
                tracer.depatch(iY * 2 + 1, iX * 3 + 2)
                tracer.depatch(iY * 2 + 1, iXright * 3 + 1)
                tracer.depatch(iY * 2 + 1, iXright * 3 + 2)
                # }

    # logger {
    logger.println('deleting the walls')
    # }
    # update deleted walls
    for i in range(width):
        for j in range(height):
            currentWall = walls[graph[i][j]]

            if currentWall['down'] is False:
                G[j * 2 + 2][i * 3 + 1] = ' '
                G[j * 2 + 2][i * 3 + 2] = ' '
                # visualize {
                tracer.select(j * 2 + 2, i * 3 + 1)
                Tracer.delay()
                tracer.select(j * 2 + 2, i * 3 + 2)
                Tracer.delay()
                # }

            if currentWall['right'] is False:
                G[j * 2 + 1][i * 3 + 3] = ' '
                # visualize {
                tracer.select(j * 2 + 1, i * 3 + 3)
                Tracer.delay()
                # }
            # visualize {
            tracer.set(G)
            # }

    # logger {
    logger.println('cleaning up the grid!')
    # }
    cleanUpGrid(width, height)

    # Clear out walls for the start and end locations.
    randomStart = random.randrange(width)
    randomEnd = random.randrange(width)

    # logger {
    logger.println('setting the Start (S) & End (E) locations')
    # }

    # Start Location
    G[0][randomStart * 3 + 1] = ' '
    G[0][randomStart * 3 + 2] = ' '
    G[1][randomStart * 3 + 1] = 'S'

    # End Location
    G[vEnd - 1][randomEnd * 3 + 1] = ' '
    G[vEnd - 1][randomEnd * 3 + 2] = ' '
    G[vEnd - 2][randomEnd * 3 + 1] = 'E'

    cleanUpStartLocation(randomStart)
    cleanUpEndLocation(randomEnd)

    # logger {
    logger.println('maze is completed!')
    # }

    # set the data
    # visualize {
    tracer.set(G)
    # }


def cleanUpStartLocation(start):
    if G[0][start * 3] == '\u252c' and G[1][start * 3] == '\u2502':
        G[0][start * 3] = '\u2510'
    if G[0][start * 3 + 3] == '\u252c' and G[1][start * 3 + 3] == '\u2502':
        G[0][start * 3 + 3] = '\u250c'
    if G[0][start * 3] == '\u250c':
        G[0][start * 3] = '\u2502'
    if G[0][start * 3 + 3] == '\u2510':
        G[0][start * 3 + 3] = '\u2502'


def cleanUpEndLocation(end):
    if G[vEnd - 1][end * 3] == '\u2534' and G[vEnd - 2][end * 3] == '\u2502':
        G[vEnd - 1][end * 3] = '\u2518'
    if G[vEnd - 1][end * 3 + 3] == '\u2534' and G[vEnd - 2][end * 3 + 3] == '\u2502':
        G[vEnd - 1][end * 3 + 3] = '\u2514'
    if G[vEnd - 1][end * 3] == '\u2514':
        G[vEnd - 1][end * 3] = '\u2502'
    if G[vEnd - 1][end * 3 + 3] == '\u2518':
        G[vEnd - 1][end * 3 + 3] = '\u2502'


def cleanUpGrid(width, height):
    # Remove room numbers
    for i in range(width):
        for j in range(height):
            G[j * 2 + 1][i * 3 + 1] = ' '
            G[j * 2 + 1][i * 3 + 2] = ' '

    # clean up grid for looks
    for i in range(vEnd):
        for j in range(hEnd):
            if G[i][j] == '\u251c':
                if G[i][j + 1] == ' ':
                    G[i][j] = '\u2502'

            if G[i][j] == '\u2524':
                if G[i][j - 1] == ' ':
                    G[i][j] = '\u2502'

            if G[i][j] == '\u252c':
                if G[i + 1][j] == ' ':
                    G[i][j] = '\u2500'

            if G[i][j] == '\u2534':
                if G[i - 1][j] == ' ':
                    G[i][j] = '\u2500'

            if G[i][j] == '\u253c':
                if G[i][j + 1] == ' ' and G[i - 1][j] == ' ' and G[i][j - 1] != ' ' and G[i + 1][j] != ' ':
                    G[i][j] = '\u2510'
                elif G[i][j - 1] == ' ' and G[i - 1][j] == ' ' and G[i + 1][j] != ' ' and G[i][j + 1] != ' ':
                    G[i][j] = '\u250c'
                elif G[i][j - 1] == ' ' and G[i + 1][j] == ' ' and G[i - 1][j] != ' ' and G[i][j + 1] != ' ':
                    G[i][j] = '\u2514'
                elif G[i][j + 1] == ' ' and G[i + 1][j] == ' ' and G[i - 1][j] != ' ' and G[i][j - 1] != ' ':
                    G[i][j] = '\u2518'
                elif G[i][j + 1] == ' ' and G[i][j - 1] == ' ' and (G[i + 1][j] == ' ' or G[i - 1][j] == ' '):
                    G[i][j] = '\u2502'
                elif G[i + 1][j] == ' ' and G[i - 1][j] == ' ' and (G[i][j - 1] == ' ' or G[i][j + 1] == ' '):
                    G[i][j] = '\u2500'
                elif G[i][j + 1] == ' ' and G[i][j - 1] == ' ':
                    G[i][j] = '\u2502'
                elif G[i + 1][j] == ' ' and G[i - 1][j] == ' ':
                    G[i][j] = '\u2500'
                elif G[i + 1][j] == ' ' and G[i - 1][j] != ' ' and G[i][j - 1] != ' ' and G[i][j + 1] != ' ':
                    G[i][j] = '\u2534'
                elif G[i - 1][j] == ' ' and G[i + 1][j] != ' ' and G[i][j + 1] != ' ' and G[i][j - 1] != ' ':
                    G[i][j] = '\u252c'
                elif G[i][j + 1] == ' ' and G[i - 1][j] != ' ' and G[i + 1][j] != ' ' and G[i][j - 1] != ' ':
                    G[i][j] = '\u2524'
                elif G[i][j - 1] == ' ' and G[i - 1][j] != ' ' and G[i + 1][j] != ' ' and G[i][j + 1] != ' ':
                    G[i][j] = '\u251c'


buildMaze()
