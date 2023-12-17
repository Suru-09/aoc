import scala.collection.mutable.Map
import scala.collection.mutable.ArrayDeque
import scala.collection.mutable.Stack
import scala.io.Source
import scala.collection.mutable.ArrayBuffer
import util.control.Breaks._

/*
    DISCLAIMER: The following solution works for PART 2 of the problem!
*/
object TuningProblem {

    def tune(file: String): Int = {
        var bufferDeque = ArrayDeque[Char]()
        var currentCount = 0
        var count = 0
        var result = 0
        // can change this value to 4 to solve PART 1
        val packetLength = 14
        breakable {
            for(line <- Source.fromFile(file).getLines) {
                line.foreach(ch => {
                    if (count == 14) {
                        if(bufferDeque.size == bufferDeque.distinct.size) {
                            break
                        }
                        count -= 1
                        bufferDeque.remove(0)
                    }
                    bufferDeque = bufferDeque :+ ch
                    count += 1
                    currentCount += 1
                })
            }
        }
        currentCount
    }

    def main(args: Array[String]): Unit = {
        if (args.length == 0) {
            throw new Exception("You should provide the input file!")
        }
        println(tune(args(0)))
    }
}