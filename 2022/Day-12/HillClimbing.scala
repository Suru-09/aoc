import scala.collection.mutable.Map
import scala.collection.mutable.ArrayDeque
import scala.collection.mutable.Stack
import scala.io.Source
import scala.collection.mutable.ArrayBuffer
import util.control.Breaks._
import scala.collection.mutable.Queue

class Hill(h: ArrayBuffer[ArrayBuffer[Char]]) {
    private val heights: ArrayBuffer[ArrayBuffer[Char]] = h

    def djikstra(source: (Int, Int)): (Array[Array[Int]], Array[Array[(Int, Int)]]) = {
        var dist: Array[Array[Int]] = Array.tabulate(heights.length, heights(0).length)(initialCellValue)
        var prev: Array[Array[(Int, Int)]] = Array.tabulate(heights.length, heights(0).length)(initPrev)
        val (x, y) = source
        dist(x)(y) = 0

        var Q: Queue[(Int, Int)] = Queue()
        for(row <- 0 to heights.length - 1) {
            for(col <- 0 to heights(0).length - 1) {
                if( (row, col) != source ) {
                    dist(row)(col) = Int.MaxValue
                    prev(row)(col) = (-1, 1)
                }
                Q.enqueue((row, col))
            }
        }

        while(!Q.isEmpty) {
            val u = Q.toSeq.minBy((x, y) => dist(x)(y))
            Q.dequeueFirst((x, y) => {
                x == u(0) && y == u(1)
            })

            for(next <- neighbors(u)) {
                val (xU, yU) = u
                val (xNext, yNext) = next
                val alt = dist(xU)(yU) + 1

                var oldVal: Char = matchOld(xU, yU)
                var newVal: Char = matchNew(xNext, yNext)

                if(alt <= dist(xNext)(yNext) && oldVal + 1 >= newVal) {
                    dist(xNext)(yNext) = alt 
                    prev(xNext)(yNext) = u
                }
            }
        }
        (dist, prev)
    }

    def matchOld(oldX: Int, oldY: Int): Char = heights(oldX)(oldY) match
        case 'S' => 'a'
        case _ => heights(oldX)(oldY)
    

    def matchNew(newX: Int, newY: Int): Char = heights(newX)(newY) match
        case 'E' => 'z'
        case _ => heights(newX)(newY) 

    def neighbors(coords: (Int, Int)): List[(Int, Int)] = {
        val (x, y) = coords
        List((x, y + 1), (x, y - 1), (x - 1, y), (x + 1, y)).filter(checkBounds)
    }

    def checkBounds(i: Int, j: Int): Boolean = {
        ((i >= 0 && i < heights.length) && (j >=0 && j < heights(0).length))
    }

    def initialCellValue(i: Int, j: Int): Int = Int.MaxValue
    def initPrev(i: Int, j: Int): (Int, Int) = (-1, -1)
}

object HillClimbing {

    def findCoords(heights: ArrayBuffer[ArrayBuffer[Char]], searched: Char): (Int, Int) = {
        var coordX = -1
        var coordY = -1
        for(row <- 0 to heights.length - 1) {
            for(col <- 0 to heights(0).length - 1) {
                if(heights(row)(col) == searched) {
                    coordX = row
                    coordY = col
                }
            }
        }
        (coordX, coordY)
    }

    def readHeights(file: String): ArrayBuffer[ArrayBuffer[Char]] = {
        var heights: ArrayBuffer[ArrayBuffer[Char]] = ArrayBuffer()
        for(line <- Source.fromFile(file).getLines) {
            var lineArr: ArrayBuffer[Char] = ArrayBuffer()
            line.foreach(ch => {
                lineArr += ch
            })
            heights += lineArr
        }
        heights
    }

    def solvePart1(file: String): Unit = {
        val heights = readHeights(file)
        var hill = Hill(heights)
        val start = findCoords(heights, 'S')
        val (x, y) = findCoords(heights, 'E')
        val result = hill.djikstra(start)
        println(s"dist: ${result(0)(x)(y)}")
    }

    def solvePart2(file: String): Unit = {
        val heights = readHeights(file)
        var hill = Hill(heights)
        var distance = Int.MaxValue
        val (x, y) = findCoords(heights, 'E')
        for(row <- 0 to heights.length - 1) {
            for(col <- 0 to heights(0).length - 1) {
                if( heights(row)(col) == 'a' || heights(row)(col) == 'S') {
                    val result = hill.djikstra((row, col))(0)(x)(y)
                    distance = if(result > 0 ) distance.min(result) else distance
                }
            }
        }
        println(s"dist: $distance")
    }

    def main(args: Array[String]): Unit = {
        if (args.length == 0) {
            throw new Exception("You should provide the input file!")
        }
        solvePart1(args(0))
        //solvePart2(args(0))
    }
}