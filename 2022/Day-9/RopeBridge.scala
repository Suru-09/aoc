import scala.collection.mutable.Map
import scala.collection.mutable.ArrayDeque
import scala.collection.mutable.Stack
import scala.io.Source
import scala.collection.mutable.ArrayBuffer
import util.control.Breaks._

/*
    DISCLAIMER: The following solution works for PART 2 of the problem!
*/

class Point(a: Int, b: Int) {
    private var x: Int = a
    private var y: Int = b

    def getX(): Int = x
    def getY(): Int = y

    def setX(newX: Int): Unit =  {
        x = newX
    }

    def setY(newY: Int): Unit =  {
        y = newY
    }

    override def equals(point: Any): Boolean = {
        point match {
            case p: Point => p.getX() == x && p.getY() == y
            case _ => false
        }
    }

    override def toString(): String = s"Point(x -> $x and y -> $y)"
}

class RopeGame(knots: Int) {
    private var rope: Array[Point] = Array.fill(knots)(Point(0, 0))
    private var tailVisited: Array[Point] = Array(Point(0, 0))

    def makeSteps(noSteps: Int, stepType: String) : Unit = {
        // this time head can only move:
        // LEFT, UP, RIGHT, DOWN
        val dx: Array[Int] = Array(0, -1, 0, 1)
        val dy: Array[Int] = Array(-1, 0, 1, 0)
        
        var lhead = rope.head
        for(i <- 0 to noSteps - 1) {
            stepType match {
                case "L" => {
                    lhead.setX(lhead.getX() + dx(0))
                    lhead.setY(lhead.getY() + dy(0))
                }
                case "U" => {
                    lhead.setX(lhead.getX() + dx(1))
                    lhead.setY(lhead.getY() + dy(1))
                }
                case "R" => {
                    lhead.setX(lhead.getX() + dx(2))
                    lhead.setY(lhead.getY() + dy(2))
                }
                case "D" => {
                    lhead.setX(lhead.getX() + dx(3))
                    lhead.setY(lhead.getY() + dy(3))
                }
                case _ => {}
            }
            
            // we check each time (rope(i) becomes tail & rope(i - 1) becomes head)
            for(i <- 1 to rope.length - 1) {
                var localHead = rope(i - 1)
                var localTail = rope(i)
                if(isPointNearHead(localTail, localHead) == false) {
                    var visitedP = moveTail(localTail, localHead)
                    rope(i) = visitedP
                    // we are only interested in the last knot path
                    if(i == rope.length - 1) {
                        appendToVisit(visitedP)
                    }  
                }
            }

            // if rope.length > 2, then this logs are becoming cumbersome
            if(rope.length == 2) {
                println(s"After [${i + 1}] <$stepType> moves: ")
                println(s"Head : ${rope.head}")
                println(s"Tail : ${rope.last}\n\n")
            }
            
        }
    }

    def appendToVisit(p: Point): Unit = {
        tailVisited = tailVisited :+ p
        tailVisited = tailVisited.distinct
    }

    def isPointNearHead(point: Point, headL: Point) : Boolean = {
        // for dx and dy the values are given in the following order:
        // diagonally up left(-1, -1) , left(0, -1), diagonally left down(1, -1)
        // up(-1, 0), down(1, 0)
        // diagonally right up(-1, 1), right(0, 1), diagonally right down(1, 1)
        val dx: Array[Int] = Array(-1, 0, 1, -1, 1, -1, 0, 1)
        val dy: Array[Int] = Array(-1, -1, -1, 0, 0, 1, 1, 1)

        var foundHead = false
        if(point == headL) {
            foundHead = true
        }

        for(i <- 0 to dx.length - 1) {
            var newPoint = Point(point.getX() + dx(i), point.getY() + dy(i))
            if (newPoint == headL) {
                foundHead = true
            }
        }  
        foundHead
    }

    private def moveTail(t: Point, h: Point) : Point = {
        val dx: Array[Int] = Array(0, -1, 0, 1)
        val dy: Array[Int] = Array(-1, 0, 1, 0)

        // LEFT UP, LEFT DOWN, RIGHT UP, RIGHT DOWN
        val diagX: Array[Int] = Array(-1, 1, -1, 1)
        val diagY: Array[Int] = Array(-1, -1, 1, 1)

        var searchFlag = true
        var point: Point = null

        // if tail must move diagonally
        if(t.getX() != h.getX() && t.getY() != h.getY() && searchFlag) {
            for(i <- 0 to diagX.length - 1) {
                var newPoint = Point(t.getX() + diagX(i), t.getY() + diagY(i))
                if (isPointNearHead(newPoint, h) && searchFlag) {
                    //println("Se intampla 1")
                    searchFlag = false
                    point = newPoint
                }
            }
        }
        // if tail must move horizontally/vertically
        else if((t.getX() == h.getX() || t.getY() == h.getY()) && searchFlag){
            for(i <- 0 to dx.length - 1) {
                var newPoint = Point(t.getX() + dx(i), t.getY() + dy(i))
                if (isPointNearHead(newPoint, h) && searchFlag) {
                    //println("Se intampla 2")
                    searchFlag = false
                    point = newPoint
                }
            }
        }
        point
    }

    def countTailVisitedPositions() : Int = {
        tailVisited = tailVisited.distinctBy(p => {
            (p.getX(), p.getY())
        })
        tailVisited.foreach(println)
        tailVisited.length
    }
}

object RopeBridge {

    def solvePart1(file: String): Int = {
        var ropeGame = RopeGame(2)
        for(line <- Source.fromFile(file).getLines) {
            var splitted = line.split(" ")
            ropeGame.makeSteps(splitted(1).toInt, splitted(0))
        }
        ropeGame.countTailVisitedPositions()
    }

    def solvePart2(file: String): Int = {
        var ropeGame = RopeGame(10)
        for(line <- Source.fromFile(file).getLines) {
            var splitted = line.split(" ")
            ropeGame.makeSteps(splitted(1).toInt, splitted(0))
        }
        ropeGame.countTailVisitedPositions()
    }


    def main(args: Array[String]): Unit = {
        if (args.length == 0) {
            throw new Exception("You should provide the input file!")
        }
        println(solvePart1(args(0)))
    }
}