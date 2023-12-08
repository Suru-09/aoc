import scala.collection.mutable.Map
import scala.io.Source

/*
    DISCLAIMER: The following solution works for PART 2 of the problem!
*/

class CleanInterval(val s: Int, e: Int) {
    if (s > e) {
        throw new IllegalArgumentException("Start must be less than or equal to end")
    }

    private val start: Int = s
    private val end: Int = e

    def isContainedBy(possiblyContained: CleanInterval): Boolean = {
        var result = false
        if (possiblyContained.getStart() <= start
                && possiblyContained.getEnd() >= end) {
            result = true
        }
        result
    }

    def doesIntersect(pIntersect: CleanInterval): Boolean = {
        var result = false
        // getStart[ start[ end] getEnd]   || start[ getStart[ getEnd] end] 
        if (this.isContainedBy(pIntersect) || pIntersect.isContainedBy(this)) 
            result = true else result = false
        
        //  start [   getStart[        end]     getEnd]
        if (pIntersect.getStart() <= end && start <= pIntersect.getStart()) {
            result = true
        }

        //  getStart [   start[        getEnd]     end]
        if (start <= pIntersect.getEnd() && pIntersect.getStart() <= start) {
            result = true
        }
        result
    }

    def getStart(): Int = start
    def getEnd(): Int = end
}


object CampCleanup {

    def splitPairs(pair: String): Array[String] = pair.split(",")


    def formatPair(pair: String) : (Int, Int) = {
        var intervals = pair.split("-")
        if (intervals.length != 2) {
            throw new IllegalArgumentException(
                "Seems that the file input doesn't respect: pair: [value-value] format!")
        }
        println(s"start: ${intervals(0)} and end: ${intervals(1)}")
        (intervals(0).toInt, intervals(1).toInt)
    }

    def cleanup(file: String): Int = {
        var countFullyContained = 0
        var countIntersections = 0
        for(line <- Source.fromFile(file).getLines) {
            var splitted = splitPairs(line)
            if (splitted.length != 2) {
                throw new IllegalArgumentException(
                    "Seems that the file input doesn't respect: [pair, pair] format!")
            }

            println("First elf: ")
            var firstElf =  formatPair(splitted(0))
            println("Second elf: ")
            var secondElf = formatPair(splitted(1))

            var firstInterval = new CleanInterval(firstElf(0), firstElf(1))
            var secondInterval = new CleanInterval(secondElf(0), secondElf(1))

            // ! Code for PART 1 (if an interval is fully contained into the other)
            /*
            if (firstInterval.isContainedBy(secondInterval) ||
                    secondInterval.isContainedBy(firstInterval)) {
                        countFullyContained += 1
            }
            */
            if(firstInterval.doesIntersect(secondInterval)) {
                countIntersections += 1
            }

        }
        countIntersections
    }


    def main(args: Array[String]): Unit = {
        if (args.length == 0) {
            throw new Exception("You should provide the input file!")
        }
        println(cleanup(args(0)))
    }
}