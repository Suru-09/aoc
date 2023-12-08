import scala.collection.mutable.Map
import scala.collection.mutable.ArrayDeque
import scala.collection.mutable.Stack
import scala.io.Source
import scala.collection.mutable.ArrayBuffer
import util.control.Breaks._

/*
    DISCLAIMER: The following solution works for PART 2 of the problem!
*/

class TreePoint(a: Int, b: Int) {
    private val x: Int = a
    private val y: Int = b

    def getX(): Int = x
    def getY(): Int = y
}

object TreetopTreeHouse {

    def readMatrix(file: String): ArrayBuffer[ArrayBuffer[Int]] = {
        var matrix: ArrayBuffer[ArrayBuffer[Int]] = ArrayBuffer()
        var count = 0
        for(line <- Source.fromFile(file).getLines) {
            var lineList = ArrayBuffer[Int]()
            line.foreach(ch => lineList = lineList :+ ch.toInt - '0')
            matrix += lineList
            count += 1
        }
        matrix
    }

    def isVisible(matrix: ArrayBuffer[ArrayBuffer[Int]], tree: TreePoint): (Boolean) = {
        var x = tree.getX()
        var y = tree.getY()
        var treeHeight = matrix(x)(y)
        var isVisible = false
        var maxi = -1

        // left
        for(i <- 0 to y - 1) {
            maxi = maxi.max(matrix(x)(i))
        }
        if(maxi < treeHeight) isVisible = true
        maxi = -1


        // right
        for(i <- y + 1 to matrix(0).length - 1) {
            maxi = maxi.max(matrix(x)(i))
        }
        if(maxi < treeHeight) isVisible = true
        maxi = -1

        // up
        for(i <- 0 to x - 1) {
            maxi = maxi.max(matrix(i)(y))
        }
        if(maxi < treeHeight) isVisible = true
        maxi = -1

        // down
        for(i <- x + 1 to matrix.length - 1) {
            maxi = maxi.max(matrix(i)(y))
        }
        if(maxi < treeHeight) isVisible = true
        isVisible
    }

    def bestVisibility(matrix: ArrayBuffer[ArrayBuffer[Int]],tree: TreePoint): Int = {
        var x = tree.getX()
        var y = tree.getY()
        var treeHeight = matrix(x)(y)
        var bestVisibility = 1
        var tmpVis = 0
        var keepLooking = true

        //println(s"FIRST Pair (x, y) -> ($x, $y)")

        // left
        for(i <- y - 1 to 0 by -1) {
            if(keepLooking && matrix(x)(i) < treeHeight) {
                tmpVis += 1
            }
            else {
                if (keepLooking) tmpVis += 1
                keepLooking = false
            }
        }
        bestVisibility *= tmpVis
        keepLooking = true; tmpVis = 0

        // right
        for(i <- y + 1 to matrix(0).length - 1) {
            if(keepLooking && matrix(x)(i) < treeHeight) {
                tmpVis += 1
            }
            else {
                if (keepLooking) tmpVis += 1
                keepLooking = false
            }
        }
        bestVisibility *= tmpVis
        keepLooking = true; tmpVis = 0
        
        // up
        for(i <- x - 1 to 0 by - 1) {
            if(keepLooking && matrix(i)(y) < treeHeight) {
                tmpVis += 1
            }
            else {
                if (keepLooking) tmpVis += 1
                keepLooking = false
            }
        }
        bestVisibility *= tmpVis
        keepLooking = true; tmpVis = 0

        // down
        for(i <- x + 1 to matrix.length - 1) {
            if(keepLooking && matrix(i)(y) < treeHeight) {
                tmpVis += 1
            }
            else {
                if (keepLooking) tmpVis += 1
                keepLooking = false
            }
        }
        bestVisibility *= tmpVis
        //println(bestVisibility)

        bestVisibility
    }

    def findVisibleTrees(file: String): Int = {
        var matrix = readMatrix(file)
        var noOfVisibleTrees = 0
        var bestVisbility = -1
        for(i <- 1 to matrix.length - 2; j <- 1 to matrix(0).length - 2) {
            //println(s"Pair (x, y) -> ($i, $j)")
            var visible = isVisible(matrix, TreePoint(i, j))
            if(visible) {
                noOfVisibleTrees += 1
            }
            bestVisbility = bestVisbility.max(bestVisibility(matrix, TreePoint(i, j)))
        }
        println(s"Best visibility: $bestVisbility")
        noOfVisibleTrees += 2 * matrix.length + 2 * (matrix(0).length - 2)
        noOfVisibleTrees
    }

    def main(args: Array[String]): Unit = {
        if (args.length == 0) {
            throw new Exception("You should provide the input file!")
        }
        println(s"Number of visible trees: ${findVisibleTrees(args(0))}")
    }
}